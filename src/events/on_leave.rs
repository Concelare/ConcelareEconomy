use std::str::FromStr;
use chrono::Utc;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerLeaveEvent};
use pumpkin_plugin_api::Server;
use tracing::error;
use uuid::Uuid;
use crate::services::database::DatabaseService;

pub struct OnLeaveEvent {
    pub(crate) db: DatabaseService
}

impl EventHandler<PlayerLeaveEvent> for OnLeaveEvent {
    fn handle(&self, _server: Server, event: <PlayerLeaveEvent as FromIntoEvent>::Data) -> <PlayerLeaveEvent as FromIntoEvent>::Data {
        let player_uuid = Uuid::from_str(event.player.get_id().as_str()).unwrap();
        if let Some(mut player) = self.db.players.get_player(player_uuid).unwrap() {
            player.last_seen = Utc::now();
            match self.db.players.save_player(&player) {
                Ok(_) => (),
                Err(e) => error!("Error saving player: {}", e)
            };
        }

        event
    }
}