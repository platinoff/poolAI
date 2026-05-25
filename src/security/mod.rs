//! Security operations (PH-S24): secret rotation hooks and JWT secret store.

pub mod jwt_secrets;
pub mod secret_rotation;

pub use jwt_secrets::{jwt_store, reload_jwt_from_env};
pub use secret_rotation::{
    init_default_rotation_hooks, register_tls_reload_hook, rotation_status, run_rotation,
    SecretKind,
};
