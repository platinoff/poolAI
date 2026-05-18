//! Dev-stand helpers (FM-003): env overrides for multi-node on one host.

/// HTTP API port (`POOLAI_HTTP_PORT`, default `8080`).
pub fn resolve_http_port() -> u16 {
    std::env::var("POOLAI_HTTP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_http_port_env() {
        const KEY: &str = "POOLAI_HTTP_PORT";
        std::env::remove_var(KEY);
        assert_eq!(resolve_http_port(), 8080);
        std::env::set_var(KEY, "9091");
        assert_eq!(resolve_http_port(), 9091);
        std::env::remove_var(KEY);
    }
}
