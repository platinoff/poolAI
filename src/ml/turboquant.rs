//! TurboQuant-style **internal** packed format (Priority 2b, Rust-only).
//!
//! Simplified per-row int8 uniform quantization with `f32` scale (PolarQuant/QJL-inspired
//! data-plane placeholder; see `docs/ml/TURBOQUANT_INTEGRATION.md`). Not wire-compatible with
//! external Google binaries — PoolAI artifact format only.

use std::fmt;

/// Magic + version tag for on-disk / in-memory blobs.
pub const FORMAT_MAGIC: &[u8; 4] = b"TQ01";
const FORMAT_VERSION: u8 = 1;

/// Target storage bits per weight after quantization (int8 payload).
pub const TARGET_BITS_PER_WEIGHT: u8 = 8;

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

fn encode_row_major(flat: &[f32], cols: u32) -> Result<Vec<u8>, TurboQuantError> {
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
        let max_abs = chunk.iter().map(|x| x.abs()).fold(0.0_f32, |a, b| a.max(b));
        let scale = if max_abs > 0.0 { max_abs / 127.0 } else { 1.0 };
        out.extend_from_slice(&scale.to_le_bytes());
        for &v in chunk {
            let qf = (v / scale).round().clamp(-127.0, 127.0);
            out.push(qf as i8 as u8);
        }
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
        for _ in 0..cols {
            let q = bytes[off] as i8;
            off += 1;
            flat.push(q as f32 * scale);
        }
    }
    Ok((flat, cols as u32))
}

/// Dot product (naive, for small vectors in tests / metrics).
pub fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
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
    fn dot_product_proxy_bounded_error() {
        let a = vec![0.3_f32, -1.1, 2.4, 0.02];
        let b = vec![-0.7_f32, 0.5, 1.0, 3.0];
        let d0 = dot_f32(&a, &b);

        let pa = pack_uniform_rows(&[a.clone()]).unwrap();
        let pb = pack_uniform_rows(&[b.clone()]).unwrap();
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
