use uuid::Uuid;

use crate::models::player::ConcelarePlayer;

pub trait PlayerRepository: Send + Sync {
    fn create_player(&self, uuid: Uuid) -> Result<ConcelarePlayer, String>;
    fn get_player(&self, uuid: Uuid) -> Result<Option<ConcelarePlayer>, String>;
    fn save_player(&self, player: &ConcelarePlayer) -> Result<(), String>;
    fn set_balance(&self, uuid: Uuid, balance: f32) -> Result<(), String>;
    fn is_frozen(&self, uuid: Uuid) -> Result<bool, String>;
    fn freeze_account(&self, uuid: Uuid) -> Result<(), String>;
    fn unfreeze_account(&self, uuid: Uuid) -> Result<(), String>;
    fn remove_money(&self, uuid: Uuid, amount: f32) -> Result<(), String>;
    fn add_money(&self, uuid: Uuid, amount: f32) -> Result<(), String>;
    fn get_balance(&self, uuid: Uuid) -> Result<f32, String>;
}