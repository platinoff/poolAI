//! In-memory user accounts (shared by HTTP auth and `AppState`).
//!
//! Production deployments should replace this with a database-backed store.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

/// User roles for role-based access control
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Operator,
    Viewer,
}

impl UserRole {
    pub fn get_permissions(&self) -> Vec<String> {
        match self {
            UserRole::Admin => vec![
                "read:all".to_string(),
                "write:all".to_string(),
                "delete:all".to_string(),
                "admin:all".to_string(),
            ],
            UserRole::Operator => vec![
                "read:all".to_string(),
                "write:workers".to_string(),
                "write:models".to_string(),
                "read:metrics".to_string(),
            ],
            UserRole::Viewer => vec![
                "read:status".to_string(),
                "read:metrics".to_string(),
                "read:models".to_string(),
            ],
        }
    }
}

/// User information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User information for API responses (without password hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub role: UserRole,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            role: user.role,
            active: user.active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Built-in administrator username seeded on first [`UserManager::initialize`].
pub const DEFAULT_DEV_ADMIN_USERNAME: &str = "admin";
/// Built-in administrator password (dev / first-run; in-memory store uses plaintext today).
pub const DEFAULT_DEV_ADMIN_PASSWORD: &str = "admin123";

/// User manager for user account management
pub struct UserManager {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
    username_index: Arc<RwLock<HashMap<String, Uuid>>>,
    initialized: Arc<RwLock<bool>>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            username_index: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn initialize(&self) -> Result<(), String> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        let default_users = vec![
            (
                DEFAULT_DEV_ADMIN_USERNAME,
                DEFAULT_DEV_ADMIN_PASSWORD,
                UserRole::Admin,
            ),
            ("operator", "op123", UserRole::Operator),
            ("viewer", "view123", UserRole::Viewer),
        ];

        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;
        let now = chrono::Utc::now();

        for (username, password, role) in default_users {
            let id = Uuid::new_v4();
            let user = User {
                id,
                username: username.to_string(),
                password_hash: password.to_string(),
                role,
                active: true,
                created_at: now,
                updated_at: now,
            };
            users.insert(id, user.clone());
            username_index.insert(username.to_string(), id);
        }

        *initialized = true;
        info!(
            "User manager initialized with {} default users",
            users.len()
        );
        Ok(())
    }

    pub async fn create_user(
        &self,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<UserInfo, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;

        if username_index.contains_key(&username) {
            return Err(format!("Username '{}' already exists", username));
        }

        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        let user = User {
            id,
            username: username.clone(),
            password_hash: password,
            role,
            active: true,
            created_at: now,
            updated_at: now,
        };

        users.insert(id, user.clone());
        username_index.insert(username, id);

        info!("Created user: {} ({})", user.username, id);
        Ok(user.into())
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<UserInfo>, String> {
        let users = self.users.read().await;
        Ok(users.get(&id).map(|u| u.clone().into()))
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let username_index = self.username_index.read().await;
        if let Some(&id) = username_index.get(username) {
            let users = self.users.read().await;
            Ok(users.get(&id).cloned())
        } else {
            Ok(None)
        }
    }

    pub async fn list_users(&self) -> Result<Vec<UserInfo>, String> {
        let users = self.users.read().await;
        Ok(users.values().map(|u| u.clone().into()).collect())
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        username: Option<String>,
        password: Option<String>,
        role: Option<UserRole>,
        active: Option<bool>,
    ) -> Result<UserInfo, String> {
        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;

        let user = users.get_mut(&id).ok_or_else(|| {
            format!("User not found: {}. Context: Cannot update non-existent user. Suggestion: Check user ID and ensure user exists.", id)
        })?;

        if let Some(new_username) = username {
            if new_username != user.username {
                if username_index.contains_key(&new_username) {
                    return Err(format!("Username '{}' already exists", new_username));
                }
                username_index.remove(&user.username);
                username_index.insert(new_username.clone(), id);
                user.username = new_username;
            }
        }

        if let Some(new_password) = password {
            user.password_hash = new_password;
        }

        if let Some(new_role) = role {
            user.role = new_role;
        }

        if let Some(new_active) = active {
            user.active = new_active;
        }

        user.updated_at = chrono::Utc::now();
        let updated_user = user.clone();

        info!("Updated user: {} ({})", updated_user.username, id);
        Ok(updated_user.into())
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), String> {
        let username = {
            let users = self.users.read().await;
            let user = users.get(&id).ok_or_else(|| {
                format!("User not found: {}. Context: Cannot delete non-existent user. Suggestion: Check user ID.", id)
            })?;
            user.username.clone()
        };

        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;

        username_index.remove(&username);
        users.remove(&id);

        info!("Deleted user: {} ({})", username, id);
        Ok(())
    }

    pub async fn verify_password(&self, username: &str, password: &str) -> Result<bool, String> {
        if let Some(user) = self.get_user_by_username(username).await? {
            if !user.active {
                return Ok(false);
            }
            Ok(user.password_hash == password)
        } else {
            Ok(false)
        }
    }

    /// `true` if this account is still the seeded admin with the default password (first-run / dev).
    pub async fn is_default_bootstrap_admin_account(&self, username: &str) -> Result<bool, String> {
        if let Some(user) = self.get_user_by_username(username).await? {
            Ok(user.username == DEFAULT_DEV_ADMIN_USERNAME
                && matches!(user.role, UserRole::Admin)
                && user.password_hash == DEFAULT_DEV_ADMIN_PASSWORD)
        } else {
            Ok(false)
        }
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}
