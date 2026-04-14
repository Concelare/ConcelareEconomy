use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::error;
use uuid::Uuid;
use crate::services::database;

pub fn freeze_command(cmd: &Command){
    let freeze_node = CommandNode::literal("freeze").execute(FreezeCommandExecutor);
    freeze_node.then(CommandNode::argument("player", &ArgumentType::Players).execute(FreezeCommandExecutor));
    cmd.then(freeze_node);
}

struct FreezeCommandExecutor;

impl CommandHandler for FreezeCommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let db = database::get_database();

        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
               
                if db.players.is_frozen(Uuid::from_str(player.get_id().as_str()).unwrap()).unwrap() {
                    let frozen_msg = TextComponent::text("This player's account is already frozen!");
                    frozen_msg.color_named(NamedColor::DarkRed);
                    frozen_msg.bold(true);
                    sender.send_message(frozen_msg);
                    return Ok(1);
                }
                
                match db.players.freeze_account(Uuid::from_str(player.get_id().as_str()).unwrap()) {
                    Ok(_) => (),
                    Err(e) => {
                        let error_msg = TextComponent::text("Error saving player!");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        error!("Error saving player: {}", e);
                        return Ok(0);
                    }
                };

                let message = TextComponent::text(format!("{}'s Account Has Been Frozen", player.get_name()).as_str());
                message.color_named(NamedColor::Green);
                message.bold(true);
                sender.send_message(message);
                return Ok(1);
            }
        }

        let missing_player_msg = TextComponent::text("Missing Argument: 'Player'");
        missing_player_msg.color_named(NamedColor::DarkRed);
        missing_player_msg.bold(true);
        sender.send_message(missing_player_msg);
        Ok(1)
    }
}