use std::sync::Mutex;
use chrono::Utc;
use redb::{Database, ReadableDatabase, TableDefinition};
use tracing::error;
use uuid::Uuid;
use crate::models::player::ConcelarePlayer;
use crate::repositories::player_repository::PlayerRepository;

const PLAYER_TABLE: TableDefinition<&str, &str> = TableDefinition::new("players");

pub struct RedbPlayerRepository {
    db: Mutex<Database>,
}

impl RedbPlayerRepository {
    pub fn new(database_path: &str) -> Result<Self, String> {
        // Create the database if it doesn't exist
        let db = Database::create(database_path).map_err(|e| e.to_string())?;

        // Create the players table if it doesn't exist
        {
            let tx = db.begin_write().map_err(|e| e.to_string())?;
            {
                let _ = tx
                    .open_table(PLAYER_TABLE)
                    .map_err(|e| e.to_string())?;
            }
            tx.commit().map_err(|e| e.to_string())?;
        }

        // Create the database instance
        Ok(Self {
            db: Mutex::new(db),
        })
    }

    fn read_player(&self, uuid: Uuid) -> Result<Option<ConcelarePlayer>, String> {
        // Get the database
        let db = self.db.lock().map_err(|e| e.to_string())?;
        // Open the players table
        let tx = db.begin_read().map_err(|e| e.to_string())?;
        let table = tx.open_table(PLAYER_TABLE).map_err(|e| e.to_string())?;

        let key = uuid.to_string();

        // Get the player from the database
        match table.get(key.as_str()).map_err(|e| e.to_string())? {
            Some(value) => {
                let value_str = value.value();
                let stored: ConcelarePlayer =
                    serde_json::from_str(value_str).map_err(|e| e.to_string())?;
                Ok(Some(stored))
            }
            None => Ok(None),
        }
    }
}

impl PlayerRepository for RedbPlayerRepository {
    fn create_player(&self, uuid: Uuid) -> Result<ConcelarePlayer, String> {
        let now = Utc::now();
        // Create the player
        let player = ConcelarePlayer {
            uuid,
            balance: 0.0,
            created_at: now,
            last_updated: now,
            last_seen: now,
            frozen: false,
        };

        // Convert the player to JSON
        let player_json = serde_json::to_string(&player).map_err(|e| e.to_string())?;

        // Get the database
        let db = self.db.lock().map_err(|e| e.to_string())?;
        let tx = db.begin_write().map_err(|e| e.to_string())?;
        {
            // Insert the player into the database
            let mut table = tx.open_table(PLAYER_TABLE).map_err(|e| e.to_string())?;
            table
                .insert(uuid.to_string().as_str(), player_json.as_str())
                .map_err(|e| e.to_string())?;
        }
        // Commit the transaction
        tx.commit().map_err(|e| e.to_string())?;

        // Return the player
        Ok(player)
    }

    fn get_player(&self, uuid: Uuid) -> Result<Option<ConcelarePlayer>, String> {
        self.read_player(uuid)
    }

    fn save_player(&self, player: &ConcelarePlayer) -> Result<(), String> {
        // Update the player's last_updated field
        let mut updated = player.clone();
        updated.last_updated = Utc::now();

        // Convert the player to JSON
        let player_json = serde_json::to_string(&updated).map_err(|e| e.to_string())?;

        // Get the database
        let db = self.db.lock().map_err(|e| e.to_string())?;
        let tx = db.begin_write().map_err(|e| e.to_string())?;
        {
            // Insert the player into the database
            let mut table = tx.open_table(PLAYER_TABLE).map_err(|e| e.to_string())?;
            table
                .insert(updated.uuid.to_string().as_str(), player_json.as_str())
                .map_err(|e| e.to_string())?;
        }
        // Commit the transaction
        tx.commit().map_err(|e| e.to_string())?;

        // Return success
        Ok(())
    }

    fn set_balance(&self, uuid: Uuid, balance: f32) -> Result<(), String> {
        // Get the player
        let mut player = self
            .read_player(uuid)?
            .ok_or_else(|| "Player not found".to_string())?;

        // Update the player's balance
        player.balance = balance;
        // Update the player's last_updated field
        player.last_updated = Utc::now();

        let player_json = serde_json::to_string(&player).map_err(|e| e.to_string())?;

        // Get the database
        let db = self.db.lock().map_err(|e| e.to_string())?;
        let tx = db.begin_write().map_err(|e| e.to_string())?;
        {
            // Insert the player into the database
            let mut table = tx.open_table(PLAYER_TABLE).map_err(|e| e.to_string())?;
            table
                .insert(uuid.to_string().as_str(), player_json.as_str())
                .map_err(|e| e.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())?;

        Ok(())
    }

    fn is_frozen(&self, uuid: Uuid) -> Result<bool, String> {
        // Get the player
        match self.read_player(uuid) {
            Ok(Some(player)) => Ok(player.frozen),
            Ok(None) => {
                let _ = self.create_player(uuid);
                Ok(self.get_player(uuid)?.map(|player| player.frozen).unwrap_or(false))
            },
            Err(e) => {
                error!("Error getting player frozen status: {}", e);
                Err("Error occurred getting player frozen status".to_string())
            },
        }
    }

    fn freeze_account(&self, uuid: Uuid) -> Result<(), String> {
        // Get the player
        let mut player =  match self.get_player(uuid) {
            Ok(Some(player)) => player,
            Ok(None) => {
                let _ = self.create_player(uuid);
                self.get_player(uuid)?.unwrap()
            }
            Err(e) => return Err(e),
        };

        // Check if the player is already frozen
        if player.frozen {
            return Err("Player is already frozen".to_string());
        }

        // Freeze the player
        player.frozen = true;

        // Save the player
        match self.save_player(&player) {
            Ok(_) => (),
            Err(e) => {
                error!("Error saving player: {}", e);
                return Err("Error occurred saving player".to_string());
            },
        };

        // Return success
        Ok(())
    }

    fn unfreeze_account(&self, uuid: Uuid) -> Result<(), String> {
        // Get the player
        let mut player = match self.get_player(uuid) {
            Ok(Some(player)) => player,
            Ok(None) => {
                let _ = self.create_player(uuid);
                self.get_player(uuid)?.unwrap()
            }
            Err(e) => return Err(e),
        };

        // Check if the player is frozen
        if !player.frozen {
            return Err("Player is not frozen".to_string());
        }

        // Unfreeze the player
        player.frozen = false;

        // Save the player
        match self.save_player(&player) {
            Ok(_) => (),
            Err(e) => {
                error!("Error saving player: {}", e);
                return Err(format!("Error saving player: {}", e));
            },
        };

        // Return success
        Ok(())
    }

    fn remove_money(&self, uuid: Uuid, amount: f32) -> Result<(), String> {
        // Get the player
        let mut player = match self.get_player(uuid) {
            Ok(Some(player)) => player,
            Ok(None) => {
                let _ = self.create_player(uuid);
                self.get_player(uuid)?.unwrap()
            }
            Err(e) => return Err(e),
        };

        // Check if the player is frozen
        if player.frozen {
            return Err("Player is frozen".to_string());
        }
        // Check if the player has enough money
        if player.balance < amount {
            return Err("Insufficient funds".to_string());
        }

        // Remove the money from the player's balance
        player.balance -= amount;

        // Save the player
        match self.save_player(&player) {
            Ok(_) => (),
            Err(e) => {
                error!("Error saving player: {}", e);
                return Err(format!("Error saving player: {}", e));
            },
        };

        // Return success
        Ok(())
    }

    fn add_money(&self, uuid: Uuid, amount: f32) -> Result<(), String> {
        // Get the player
        let mut player = match self.read_player(uuid) {
            Ok(Some(player)) => player,
            Ok(None) => {
                let _ = self.create_player(uuid);
                self.get_player(uuid)?.unwrap()
            }
            Err(e) => return Err(e),
        };

        if player.frozen {
            return Err("Player is frozen".to_string());
        }

        // Add the money to the player's balance
        player.balance += amount;

        // Save the player
        match self.save_player(&player) {
            Ok(_) => (),
            Err(e) => {
                error!("Error saving player: {}", e);
                return Err("Error occurred saving player".to_string());
            },
        };

        // Return success
        Ok(())
    }

    fn get_balance(&self, uuid: Uuid) -> Result<f32, String> {
        match self.read_player(uuid) {
            Ok(Some(player)) => Ok(player.balance),
            Ok(None) => {
                let _ = self.create_player(uuid);
                Ok(self.get_player(uuid)?.map(|player| player.balance).unwrap_or(0.0))
            },
            Err(e) => {
                error!("Error getting player balance: {}", e);
                Err("Error occurred getting player balance".to_string())
            },
        }
    }
}