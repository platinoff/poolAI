//! TurboQuant-style **internal** packed format (Priority 2b, Rust-only).
//!
//! Simplified per-row int8 uniform quantization with `f32` scale (PolarQuant/QJL-inspired
//! data-plane placeholder; see `docs/ml/TURBOQUANT_INTEGRATION.md`). Not wire-compatible with
//! external Google binaries — PoolAI artifact format only.
//!
//! Hot loops use a **portable fast path** (4-wide unrolling; pack uses `inv_scale`; unpack uses
//! batched i8→f32×scale). With **`--features turboquant-simd`**, the same loops use `wide::f32x4`
//! (FM-004 / Horizon S35). Default build stays scalar-only — no nightly `portable_simd`.

use std::fmt;

/// Magic + version tag for on-disk / in-memory blobs.
pub const FORMAT_MAGIC: &[u8; 4] = b"TQ01";
const FORMAT_VERSION: u8 = 1;

/// Target storage bits per weight after quantization (int8 payload).
pub const TARGET_BITS_PER_WEIGHT: u8 = 8;

/// `true` when this binary was built with `turboquant-simd` (FM-004).
#[inline]
pub const fn simd_fast_path_enabled() -> bool {
    cfg!(feature = "turboquant-simd")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurboQuantError {
    EmptyRows,
    RaggedRows,
    ZeroColumns,
    BadMagic,
    UnsupportedVersion(u8),
    Truncated,
    SizeMismatch,
}

impl fmt::Display for TurboQuantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRows => write!(f, "turboquant: no rows"),
            Self::RaggedRows => write!(f, "turboquant: rows have different lengths"),
            Self::ZeroColumns => write!(f, "turboquant: columns must be > 0"),
            Self::BadMagic => write!(f, "turboquant: invalid magic / corrupt blob"),
            Self::UnsupportedVersion(v) => write!(f, "turboquant: unsupported format version {v}"),
            Self::Truncated => write!(f, "turboquant: truncated blob"),
            Self::SizeMismatch => write!(f, "turboquant: declared size does not match payload"),
        }
    }
}

impl std::error::Error for TurboQuantError {}

/// Result of packing float rows into the internal TurboQuant blob.
#[derive(Debug)]
pub struct TurboQuantPackResult {
    pub bytes: Vec<u8>,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub target_bits: u8,
    pub rows: u32,
    pub cols: u32,
}

/// Uniform int8 quantization per row: `scale = max_abs / 127`, stored as `f32` + `cols` bytes.
pub fn pack_uniform_rows(rows: &[Vec<f32>]) -> Result<TurboQuantPackResult, TurboQuantError> {
    if rows.is_empty() {
        return Err(TurboQuantError::EmptyRows);
    }
    let cols = rows[0].len();
    if cols == 0 {
        return Err(TurboQuantError::ZeroColumns);
    }
    if rows.iter().any(|r| r.len() != cols) {
        return Err(TurboQuantError::RaggedRows);
    }

    let nrows = rows.len();
    let mut flat: Vec<f32> = Vec::with_capacity(nrows * cols);
    for r in rows {
        flat.extend_from_slice(r);
    }

    let bytes_in = (flat.len() * std::mem::size_of::<f32>()) as u64;
    let encoded = encode_row_major(&flat, cols as u32)?;
    let bytes_out = encoded.len() as u64;

    Ok(TurboQuantPackResult {
        bytes: encoded,
        bytes_in,
        bytes_out,
        target_bits: TARGET_BITS_PER_WEIGHT,
        rows: nrows as u32,
        cols: cols as u32,
    })
}

/// Decode blob produced by [`encode_row_major`] / [`pack_uniform_rows`].
pub fn unpack_to_rows(bytes: &[u8]) -> Result<Vec<Vec<f32>>, TurboQuantError> {
    let (flat, cols) = decode_row_major(bytes)?;
    let cols_u = cols as usize;
    if flat.len() % cols_u != 0 {
        return Err(TurboQuantError::SizeMismatch);
    }
    let mut out = Vec::with_capacity(flat.len() / cols_u);
    for chunk in flat.chunks(cols_u) {
        out.push(chunk.to_vec());
    }
    Ok(out)
}

#[inline]
fn row_max_abs(values: &[f32]) -> f32 {
    #[cfg(feature = "turboquant-simd")]
    {
        return row_max_abs_simd(values);
    }
    #[cfg(not(feature = "turboquant-simd"))]
    {
        row_max_abs_scalar(values)
    }
}

#[cfg_attr(feature = "turboquant-simd", allow(dead_code))]
#[inline]
fn row_max_abs_scalar(values: &[f32]) -> f32 {
    let n = values.len();
    let mut m = 0.0_f32;
    let mut i = 0;
    while i + 4 <= n {
        let a0 = values[i].abs();
        let a1 = values[i + 1].abs();
        let a2 = values[i + 2].abs();
        let a3 = values[i + 3].abs();
        m = m.max(a0.max(a1).max(a2.max(a3)));
        i += 4;
    }
    while i < n {
        m = m.max(values[i].abs());
        i += 1;
    }
    m
}

#[cfg(feature = "turboquant-simd")]
#[inline]
fn row_max_abs_simd(values: &[f32]) -> f32 {
    use wide::f32x4;

    let n = values.len();
    let mut m = f32x4::splat(0.0);
    let mut i = 0;
    while i + 4 <= n {
        let chunk = f32x4::new([values[i], values[i + 1], values[i + 2], values[i + 3]]);
        m = m.max(chunk.abs());
        i += 4;
    }
    let mut scalar = m.to_array().into_iter().fold(0.0_f32, |a, b| a.max(b));
    while i < n {
        scalar = scalar.max(values[i].abs());
        i += 1;
    }
    scalar
}

#[inline]
fn append_quantized_row(out: &mut Vec<u8>, row: &[f32], inv_scale: f32) {
    #[cfg(feature = "turboquant-simd")]
    {
        append_quantized_row_simd(out, row, inv_scale);
        return;
    }
    #[cfg(not(feature = "turboquant-simd"))]
    {
        append_quantized_row_scalar(out, row, inv_scale);
    }
}

#[cfg_attr(feature = "turboquant-simd", allow(dead_code))]
#[inline]
fn append_quantized_row_scalar(out: &mut Vec<u8>, row: &[f32], inv_scale: f32) {
    let n = row.len();
    let mut i = 0;
    while i + 4 <= n {
        for j in 0..4 {
            let qf = (row[i + j] * inv_scale).round().clamp(-127.0, 127.0);
            out.push(qf as i8 as u8);
        }
        i += 4;
    }
    while i < n {
        let qf = (row[i] * inv_scale).round().clamp(-127.0, 127.0);
        out.push(qf as i8 as u8);
        i += 1;
    }
}

#[cfg(feature = "turboquant-simd")]
#[inline]
fn append_quantized_row_simd(out: &mut Vec<u8>, row: &[f32], inv_scale: f32) {
    use wide::f32x4;

    let inv = f32x4::splat(inv_scale);
    let lo = f32x4::splat(-127.0);
    let hi = f32x4::splat(127.0);
    let n = row.len();
    let mut i = 0;
    while i + 4 <= n {
        let v = f32x4::new([row[i], row[i + 1], row[i + 2], row[i + 3]]) * inv;
        let rounded = v.round();
        let q = rounded.max(lo).min(hi).to_array();
        for qf in q {
            out.push(qf as i8 as u8);
        }
        i += 4;
    }
    while i < n {
        let qf = (row[i] * inv_scale).round().clamp(-127.0, 127.0);
        out.push(qf as i8 as u8);
        i += 1;
    }
}

/// Dequantise one row of `cols` int8 weights into `flat`; returns new `off` past consumed bytes.
#[inline]
fn push_dequantized_row(
    flat: &mut Vec<f32>,
    bytes: &[u8],
    off: usize,
    cols: usize,
    scale: f32,
) -> usize {
    #[cfg(feature = "turboquant-simd")]
    {
        return push_dequantized_row_simd(flat, bytes, off, cols, scale);
    }
    #[cfg(not(feature = "turboquant-simd"))]
    {
        push_dequantized_row_scalar(flat, bytes, off, cols, scale)
    }
}

#[cfg_attr(feature = "turboquant-simd", allow(dead_code))]
#[inline]
fn push_dequantized_row_scalar(
    flat: &mut Vec<f32>,
    bytes: &[u8],
    mut off: usize,
    cols: usize,
    scale: f32,
) -> usize {
    let mut col = 0;
    while col + 4 <= cols {
        flat.push(bytes[off] as i8 as f32 * scale);
        flat.push(bytes[off + 1] as i8 as f32 * scale);
        flat.push(bytes[off + 2] as i8 as f32 * scale);
        flat.push(bytes[off + 3] as i8 as f32 * scale);
        off += 4;
        col += 4;
    }
    while col < cols {
        flat.push(bytes[off] as i8 as f32 * scale);
        off += 1;
        col += 1;
    }
    off
}

#[cfg(feature = "turboquant-simd")]
#[inline]
fn push_dequantized_row_simd(
    flat: &mut Vec<f32>,
    bytes: &[u8],
    mut off: usize,
    cols: usize,
    scale: f32,
) -> usize {
    use wide::f32x4;

    let s = f32x4::splat(scale);
    let mut col = 0;
    while col + 4 <= cols {
        let q = f32x4::new([
            bytes[off] as i8 as f32,
            bytes[off + 1] as i8 as f32,
            bytes[off + 2] as i8 as f32,
            bytes[off + 3] as i8 as f32,
        ]);
        let deq = (q * s).to_array();
        flat.extend_from_slice(&deq);
        off += 4;
        col += 4;
    }
    while col < cols {
        flat.push(bytes[off] as i8 as f32 * scale);
        off += 1;
        col += 1;
    }
    off
}

fn encode_row_major(flat: &[f32], cols: u32) -> Result<Vec<u8>, TurboQuantError> {
    if cols == 0 {
        return Err(TurboQuantError::ZeroColumns);
    }
    let c = cols as usize;
    if !flat.len().is_multiple_of(c) {
        return Err(TurboQuantError::SizeMismatch);
    }
    let rows = flat.len() / c;

    let row_bytes = 4 + c;
    let mut out = Vec::with_capacity(4 + 1 + 4 + 4 + rows * row_bytes);
    out.extend_from_slice(FORMAT_MAGIC);
    out.push(FORMAT_VERSION);
    out.extend_from_slice(&cols.to_le_bytes());
    out.extend_from_slice(&(rows as u32).to_le_bytes());

    for chunk in flat.chunks(c) {
        let max_abs = row_max_abs(chunk);
        let scale = if max_abs > 0.0 { max_abs / 127.0 } else { 1.0 };
        out.extend_from_slice(&scale.to_le_bytes());
        let inv_scale = 1.0 / scale;
        append_quantized_row(&mut out, chunk, inv_scale);
    }
    Ok(out)
}

fn decode_row_major(bytes: &[u8]) -> Result<(Vec<f32>, u32), TurboQuantError> {
    if bytes.len() < 13 {
        return Err(TurboQuantError::Truncated);
    }
    if &bytes[0..4] != FORMAT_MAGIC {
        return Err(TurboQuantError::BadMagic);
    }
    let ver = bytes[4];
    if ver != FORMAT_VERSION {
        return Err(TurboQuantError::UnsupportedVersion(ver));
    }
    let cols = u32::from_le_bytes(bytes[5..9].try_into().unwrap()) as usize;
    if cols == 0 {
        return Err(TurboQuantError::ZeroColumns);
    }
    let nrows = u32::from_le_bytes(bytes[9..13].try_into().unwrap()) as usize;

    let row_bytes = 4 + cols;
    let expected = 13 + nrows * row_bytes;
    if bytes.len() != expected {
        return Err(TurboQuantError::SizeMismatch);
    }

    let mut flat = Vec::with_capacity(nrows * cols);
    let mut off = 13;
    for _ in 0..nrows {
        if off + 4 > bytes.len() {
            return Err(TurboQuantError::Truncated);
        }
        let scale = f32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        off += 4;
        off = push_dequantized_row(&mut flat, bytes, off, cols, scale);
    }
    Ok((flat, cols as u32))
}

/// Dot product (4-wide; SIMD when `turboquant-simd` is enabled).
pub fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
    #[cfg(feature = "turboquant-simd")]
    {
        return dot_f32_simd(a, b);
    }
    #[cfg(not(feature = "turboquant-simd"))]
    {
        dot_f32_scalar(a, b)
    }
}

#[cfg_attr(feature = "turboquant-simd", allow(dead_code))]
#[inline]
fn dot_f32_scalar(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len().min(b.len());
    let mut sum = 0.0_f32;
    let mut i = 0;
    while i + 4 <= n {
        sum += a[i] * b[i] + a[i + 1] * b[i + 1] + a[i + 2] * b[i + 2] + a[i + 3] * b[i + 3];
        i += 4;
    }
    while i < n {
        sum += a[i] * b[i];
        i += 1;
    }
    sum
}

#[cfg(feature = "turboquant-simd")]
#[inline]
fn dot_f32_simd(a: &[f32], b: &[f32]) -> f32 {
    use wide::f32x4;

    let n = a.len().min(b.len());
    let mut sum = f32x4::splat(0.0);
    let mut i = 0;
    while i + 4 <= n {
        let va = f32x4::new([a[i], a[i + 1], a[i + 2], a[i + 3]]);
        let vb = f32x4::new([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        sum += va * vb;
        i += 4;
    }
    let mut scalar = sum.to_array().into_iter().fold(0.0_f32, |a, b| a + b);
    while i < n {
        scalar += a[i] * b[i];
        i += 1;
    }
    scalar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_small_matrix() {
        let rows = vec![vec![1.0_f32, -2.0, 0.25], vec![0.0, 100.0, -50.0]];
        let p = pack_uniform_rows(&rows).unwrap();
        let back = unpack_to_rows(&p.bytes).unwrap();
        assert_eq!(back.len(), rows.len());
        for (orig, got) in rows.iter().zip(&back) {
            for (o, g) in orig.iter().zip(got) {
                let tol = (o.abs() * 0.02).max(5e-3);
                assert!(
                    (o - g).abs() <= tol,
                    "reconstruction error too large: o={o} g={g} tol={tol}"
                );
            }
        }
    }

    #[test]
    fn round_trip_five_columns_unroll_tail() {
        let rows: Vec<Vec<f32>> = (0..4)
            .map(|i| (0..5).map(|j| ((i * 5 + j) as f32) * 0.07 - 0.5).collect())
            .collect();
        let p = pack_uniform_rows(&rows).unwrap();
        let back = unpack_to_rows(&p.bytes).unwrap();
        assert_eq!(back.len(), rows.len());
        for (orig, got) in rows.iter().zip(&back) {
            for (o, g) in orig.iter().zip(got) {
                let tol = (o.abs() * 0.02).max(5e-3);
                assert!((o - g).abs() <= tol, "reconstruction error: o={o} g={g}");
            }
        }
    }

    #[test]
    fn dot_product_proxy_bounded_error() {
        let a = vec![0.3_f32, -1.1, 2.4, 0.02];
        let b = vec![-0.7_f32, 0.5, 1.0, 3.0];
        let d0 = dot_f32(&a, &b);

        let pa = pack_uniform_rows(std::slice::from_ref(&a)).unwrap();
        let pb = pack_uniform_rows(std::slice::from_ref(&b)).unwrap();
        let ra = unpack_to_rows(&pa.bytes).unwrap().pop().unwrap();
        let rb = unpack_to_rows(&pb.bytes).unwrap().pop().unwrap();
        let d1 = dot_f32(&ra, &rb);

        let denom = d0.abs().max(1e-4);
        let rel = (d0 - d1).abs() / denom;
        assert!(
            rel < 0.08,
            "relative dot error {rel} too large (d0={d0}, d1={d1})"
        );
    }

    #[test]
    fn bad_magic_fails() {
        let mut b = pack_uniform_rows(&[vec![1.0_f32]]).unwrap().bytes;
        b[0] = b'X';
        assert!(matches!(unpack_to_rows(&b), Err(TurboQuantError::BadMagic)));
    }

    #[cfg(feature = "turboquant-simd")]
    #[test]
    fn simd_pack_matches_scalar_reference() {
        let rows: Vec<Vec<f32>> = (0..6)
            .map(|i| (0..7).map(|j| ((i * 7 + j) as f32) * 0.11 - 2.0).collect())
            .collect();
        let flat: Vec<f32> = rows.iter().flatten().copied().collect();
        let cols = rows[0].len() as u32;

        let simd_bytes = super::encode_row_major(&flat, cols).expect("simd encode");
        let scalar_bytes = encode_row_major_scalar(&flat, cols).expect("scalar encode");
        assert_eq!(simd_bytes, scalar_bytes);

        let simd_back = unpack_to_rows(&simd_bytes).unwrap();
        let scalar_back = unpack_to_rows(&scalar_bytes).unwrap();
        assert_eq!(simd_back, scalar_back);
    }

    /// Scalar-only encode for parity test when `turboquant-simd` is on.
    #[cfg(feature = "turboquant-simd")]
    fn encode_row_major_scalar(flat: &[f32], cols: u32) -> Result<Vec<u8>, TurboQuantError> {
        if cols == 0 {
            return Err(TurboQuantError::ZeroColumns);
        }
        let c = cols as usize;
        if flat.len() % c != 0 {
            return Err(TurboQuantError::SizeMismatch);
        }
        let rows = flat.len() / c;
        let row_bytes = 4 + c;
        let mut out = Vec::with_capacity(4 + 1 + 4 + 4 + rows * row_bytes);
        out.extend_from_slice(FORMAT_MAGIC);
        out.push(FORMAT_VERSION);
        out.extend_from_slice(&cols.to_le_bytes());
        out.extend_from_slice(&(rows as u32).to_le_bytes());
        for chunk in flat.chunks(c) {
            let max_abs = row_max_abs_scalar(chunk);
            let scale = if max_abs > 0.0 { max_abs / 127.0 } else { 1.0 };
            out.extend_from_slice(&scale.to_le_bytes());
            let inv_scale = 1.0 / scale;
            append_quantized_row_scalar(&mut out, chunk, inv_scale);
        }
        Ok(out)
    }

    #[test]
    fn compression_ratio_reasonable() {
        let rows: Vec<Vec<f32>> = (0..8)
            .map(|i| (0..16).map(|j| (i * j) as f32 * 0.01).collect())
            .collect();
        let p = pack_uniform_rows(&rows).unwrap();
        assert!(p.bytes_out < p.bytes_in);
        assert_eq!(p.target_bits, TARGET_BITS_PER_WEIGHT);
    }
}
