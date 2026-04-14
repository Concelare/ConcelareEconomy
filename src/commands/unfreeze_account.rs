use std::str::FromStr;
use command::ConsumedArgs;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::{command, Server};
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::error;
use uuid::Uuid;
use crate::services::database;

pub fn unfreeze_command(cmd: &Command) {
    let unfreeze_node = CommandNode::literal("unfreeze");
    unfreeze_node.then(CommandNode::argument("player", &ArgumentType::Players).execute(UnfreezeCommandExecutor));

    cmd.then(unfreeze_node);
}

struct UnfreezeCommandExecutor;

impl CommandHandler for UnfreezeCommandExecutor {
    fn handle(
        &self,
        sender: command::CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let db = database::get_database();

        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let uuid = Uuid::from_str(player.get_id().as_str()).unwrap();

                if !db.players.is_frozen(uuid).unwrap() {
                    let frozen_msg = TextComponent::text("This player's account is not frozen!");
                    frozen_msg.color_named(NamedColor::DarkRed);
                    frozen_msg.bold(true);
                    sender.send_message(frozen_msg);
                    return Ok(1);
                }

                match db.players.unfreeze_account(uuid) {
                    Ok(_) => (),
                    Err(e) => {
                        let error_msg = TextComponent::text("Error unfreezing player!");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        error!("Error unfreezing player: {}", e);
                        return Ok(1);
                    }
                }

                let message = TextComponent::text(format!("{}'s Account Has Been Unfrozen", player.get_name()).as_str());
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