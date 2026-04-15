use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use uuid::Uuid;
use crate::services::database;
use crate::util::numbers::format_money;

pub fn account_command(cmd: &Command) {
    let node = CommandNode::literal("account").execute(AccountCommandExecutor);
    let player_argument = CommandNode::argument("player", &ArgumentType::Players);
    node.then(player_argument.execute(AccountCommandExecutor));

    cmd.then(node);
}

pub struct AccountCommandExecutor;

impl CommandHandler for AccountCommandExecutor {
    fn handle(&self, sender: CommandSender, server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let db = database::get_database();
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let player_uuid = match Uuid::from_str(player.get_id().as_str()) {
                    Ok(uuid) => uuid,
                    Err(_) => continue
                };
                let player_info = match db.players.get_player(player_uuid) {
                    Ok(Some(player_info)) => player_info,
                    Ok(None) => {
                        let msg = TextComponent::text("Player not found");
                        msg.color_named(NamedColor::DarkRed);
                        msg.bold(true);
                        sender.send_message(msg);
                        return Ok(1);
                    },
                    Err(err) => {
                        let error_msg = TextComponent::text("Failed to retrieve player information");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        return Ok(0);
                    },
                };

                let msg = TextComponent::text(format!("{}'s Information", player.get_name()).as_str());
                msg.color_named(NamedColor::Aqua);
                msg.bold(true);
                sender.send_message(msg);
                let msg_2 = TextComponent::text(format!("Username: {}", player.get_name()).as_str());
                msg_2.color_named(NamedColor::White);
                sender.send_message(msg_2);
                let msg_3 = TextComponent::text(format!("UUID: {}", player_uuid).as_str());
                msg_3.color_named(NamedColor::White);
                sender.send_message(msg_3);
                let msg_4 = TextComponent::text(format!("Balance: ${}", format_money(player_info.balance)).as_str());
                msg_4.color_named(NamedColor::White);
                sender.send_message(msg_4);
                let frozen = match player_info.frozen {
                    true => "Yes",
                    false => "No"
                };
                let msg_5 = TextComponent::text(format!("Frozen: {}", frozen).as_str());
                msg_5.color_named(NamedColor::White);
                sender.send_message(msg_5);
                let mut msg_6 = TextComponent::text(format!("Last Seen: {}", player_info.last_seen).as_str());

                if server.get_player_by_uuid(player.get_id().as_str()).is_some() {
                    msg_6 = TextComponent::text("Last Seen: Online");
                }

                msg_6.color_named(NamedColor::White);
                sender.send_message(msg_6);

                return Ok(1);
            }
        }

        let missing_msg = TextComponent::text("Missing Argument: 'Player'");
        missing_msg.color_named(NamedColor::DarkRed);
        missing_msg.bold(true);
        sender.send_message(missing_msg);

        Ok(1)
    }
}