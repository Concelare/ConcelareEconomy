use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConcelarePlayer {
    pub uuid: Uuid,
    pub balance: f32,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub frozen: bool,
}