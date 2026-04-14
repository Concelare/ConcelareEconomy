use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub sender: Uuid,
    pub receiver: Uuid,
    pub amount: f32,
    pub timestamp: DateTime<Utc>,
}