use std::str::FromStr;
use pumpkin_plugin_api::command_wit::Player;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerJoinEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::{error, info};
use uuid::Uuid;
use crate::services::database::DatabaseService;

pub struct OnJoinEvent {
    pub(crate) db: DatabaseService
}

impl EventHandler<PlayerJoinEvent> for OnJoinEvent {
    fn handle(&self, _server: Server, event: <PlayerJoinEvent as FromIntoEvent>::Data) -> <PlayerJoinEvent as FromIntoEvent>::Data {
        let player: &Player = &event.player;

        let id = player.get_id();

        match self.db.players.get_player(Uuid::from_str(id.as_str()).unwrap()) {
            Ok(player) => {
                if player.is_none() {
                    let _ = self.db.players.create_player(Uuid::from_str(id.as_str()).unwrap());
                    info!("Created new player: {}", id);
                }
            },
            Err(e) => {
                error!("Error getting player: {}", e);
                return event;
            }
        }


        let _ = event.player.send_system_message(TextComponent::text("Welcome to the server!"), true);

        event
    }
}