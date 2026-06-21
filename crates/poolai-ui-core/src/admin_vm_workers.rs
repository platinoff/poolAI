//! Admin vm/workers API shape helpers (PH-S823/S824, Galaxy §2.3 admin subset).

use serde_json::Value;

/// Required keys on each `GET /api/v1/workers` row for admin wasm panel (Galaxy §2.3 admin subset).
pub const WORKERS_ADMIN_ROW_KEYS: &[&str] =
    &["id", "status", "is_healthy", "total_requests_processed"];

/// Required keys on each `GET /api/v1/vm/instances` row for admin wasm panel.
pub const VM_INSTANCE_ADMIN_ROW_KEYS: &[&str] = &["id", "name", "status", "resources"];

/// Validate workers list JSON array shape for stand smoke / admin glue (PH-S824).
pub fn validate_workers_admin_list_shape(body: &Value) -> Result<(), String> {
    let arr = body
        .as_array()
        .ok_or_else(|| format!("workers body not array: {body}"))?;
    if let Some(first) = arr.first() {
        let o = first
            .as_object()
            .ok_or_else(|| format!("worker row not object: {first}"))?;
        for key in WORKERS_ADMIN_ROW_KEYS {
            if !o.contains_key(*key) {
                return Err(format!("worker row missing `{key}`: {o:?}"));
            }
        }
    }
    Ok(())
}

/// Validate VM instances list JSON array shape for stand smoke / admin glue (PH-S824).
pub fn validate_vm_instances_admin_list_shape(body: &Value) -> Result<(), String> {
    let arr = body
        .as_array()
        .ok_or_else(|| format!("vm instances body not array: {body}"))?;
    if let Some(first) = arr.first() {
        let o = first
            .as_object()
            .ok_or_else(|| format!("vm instance row not object: {first}"))?;
        for key in VM_INSTANCE_ADMIN_ROW_KEYS {
            if !o.contains_key(*key) {
                return Err(format!("vm instance row missing `{key}`: {o:?}"));
            }
        }
        let resources = o
            .get("resources")
            .and_then(|v| v.as_object())
            .ok_or_else(|| format!("vm instance missing resources object: {o:?}"))?;
        for key in ["cpu_cores", "memory_mb"] {
            if !resources.contains_key(key) {
                return Err(format!("vm resources missing `{key}`: {resources:?}"));
            }
        }
    }
    Ok(())
}

/// Galaxy §2.3 admin telemetry subset from workers API row (PH-S824 concept stub).
pub fn admin_worker_galaxy_telemetry_subset(row: &Value) -> Option<u32> {
    let healthy = row.get("is_healthy").and_then(|v| v.as_bool())?;
    if !healthy {
        return None;
    }
    row.get("total_requests_processed")
        .and_then(|v| v.as_u64())
        .map(|n| n.min(u32::MAX as u64) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_workers_admin_list_shape_ph_s824() {
        let body = json!([{
            "id": "w1",
            "status": "idle",
            "is_healthy": true,
            "total_requests_processed": 3
        }]);
        validate_workers_admin_list_shape(&body).expect("workers shape");
        assert_eq!(admin_worker_galaxy_telemetry_subset(&body[0]), Some(3));
    }

    #[test]
    fn validate_vm_instances_admin_list_shape_ph_s824() {
        let body = json!([{
            "id": "vm-1",
            "name": "test",
            "status": "running",
            "resources": { "cpu_cores": 2, "memory_mb": 1024 }
        }]);
        validate_vm_instances_admin_list_shape(&body).expect("vm shape");
    }
}
