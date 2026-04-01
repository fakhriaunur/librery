use super::ActorId;

/// Patron credentials for authentication.
///
/// Composition (has-a) relationship with Actor via actor_id FK.
/// Core stores only the password_hash (String) — hashing is done in shell (bcrypt).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatronCredentials {
    pub actor_id: ActorId,
    pub password_hash: String,
    pub created_at: i64,
    pub last_login: Option<i64>,
}
