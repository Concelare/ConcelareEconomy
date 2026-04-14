use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use uuid::Uuid;
use crate::services::transaction::TRANSACTIONS;

pub fn log_command(cmd: &Command) {
    let log_node =  CommandNode::literal("log").execute(LogCommandExecutor);
    let player_arg = ArgumentType::Players;

    log_node.then({
        let node = CommandNode::argument("player", &player_arg).execute(LogCommandExecutor);
        node
    });

    cmd.then(log_node);
}

struct LogCommandExecutor;

impl CommandHandler for LogCommandExecutor {
    fn handle(&self, sender: CommandSender, server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let transaction_service = TRANSACTIONS.get().unwrap();
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let player_uuid = Uuid::from_str(player.get_id().as_str()).unwrap();
                let logs = match transaction_service.search_by_participant(player_uuid) {
                    Ok(logs) => logs,
                    Err(_) => {
                        let msg = TextComponent::text("Unable to fetch logs");
                        msg.color_named(NamedColor::DarkRed);
                        sender.send_message(msg);
                        return Ok(0);
                    }
                };

                let msg = TextComponent::text(&format!("{}'s transactions:", player.get_name()));
                msg.color_named(NamedColor::Green);
                msg.bold(true);
                sender.send_message(msg);
                let mut i = 0;

                for log in logs {
                    let msg = TextComponent::text(&format!("{}: ${} - From {} to {}", log.timestamp.format("%Y-%m-%d %H:%M:%S"), log.amount,server.get_player_by_uuid(log.sender.as_simple().to_string().as_str()).unwrap().get_name(), server.get_player_by_uuid(log.receiver.as_simple().to_string().as_str()).unwrap().get_name()));
                    msg.color_named(NamedColor::White);
                    sender.send_message(msg);
                    i += 1;
                    if i == 20 {
                        break;
                    }
                }
                return Ok(1);
            }
        }

        let logs = match transaction_service.read_all() {
            Ok(logs) => logs,
            Err(_) => {
                let msg = TextComponent::text("Unable to fetch logs");
                msg.color_named(NamedColor::DarkRed);
                sender.send_message(msg);
                return Ok(0);
            }
        };
        let msg = TextComponent::text("All transactions:");
        msg.color_named(NamedColor::Green);
        msg.bold(true);
        sender.send_message(msg);

        let mut i = 0;
        for log in logs {
            let msg = TextComponent::text(&format!("{}: ${} - From {} to {}", log.timestamp.format("%Y-%m-%d %H:%M:%S"), log.amount,server.get_player_by_uuid(log.sender.as_simple().to_string().as_str()).unwrap().get_name(), server.get_player_by_uuid(log.receiver.as_simple().to_string().as_str()).unwrap().get_name()));
            msg.color_named(NamedColor::White);
            sender.send_message(msg);
            i += 1;
            if i == 20 {
                break;
            }
        }

        Ok(1)
    }
}