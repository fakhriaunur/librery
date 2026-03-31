use super::ActorId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub id: ActorId,
    pub actor_type: ActorType,
    pub name: String,
    pub email: Option<String>,
    pub status: ActorStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorType {
    Patron,
    Librarian,
    Admin,
    Machine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorStatus {
    Active,
    Suspended,
    Blocked,
    Expired,
}
