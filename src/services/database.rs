use std::sync::{Arc, OnceLock};

use crate::repositories::player_repository::PlayerRepository;
use crate::repositories::redb_player_repository::RedbPlayerRepository;

pub static DATABASE: OnceLock<Arc<DatabaseService>> = OnceLock::new();

#[derive(Clone)]
pub struct DatabaseService {
    pub players: Arc<dyn PlayerRepository>,
}

impl DatabaseService {
    pub fn new(database_path: &str) -> Result<Self, String> {
        let players = Arc::new(RedbPlayerRepository::new(database_path)?);

        match DATABASE.set(Arc::new(Self { players: players.clone() })) {
            Ok(_) => (),
            Err(_) => return Err("Database already initialized".to_string()),
        };

        Ok(Self { players })
    }
}

pub fn get_database() -> Arc<DatabaseService> {
    DATABASE.get().unwrap().clone()
}