use std::str::FromStr;
use chrono::Utc;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, Number};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::error;
use uuid::Uuid;
use crate::models::transaction::Transaction;
use crate::services::{database, transaction};
use crate::util::numbers::format_money;

pub fn pay_command() -> Command {
    let names = ["pay".to_string()];
    let description = "Pay another player";

    let cmd = Command::new(&names, description);

    let player_arg = ArgumentType::Players;
    let arg = ArgumentType::Float((Some(0f32), Some(f32::MAX)));
    cmd.then({
        let node = CommandNode::argument("player", &player_arg);
        node.then(CommandNode::argument("amount", &arg).execute(PayCommandExecutor));
        node
    });
    cmd.execute(PayCommandExecutor)
}

struct PayCommandExecutor;

impl CommandHandler for PayCommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let db = database::get_database();
        let transaction_log = transaction::TRANSACTIONS.get().unwrap();

        if let Arg::Players(players) = args.get_value("player") {
            for player in players {

                let sender_uuid = Uuid::from_str(sender.as_player().unwrap().get_id().as_str()).unwrap();
                let target_uuid = Uuid::from_str(player.get_id().as_str()).unwrap();
                let balance = match db.players.get_balance(sender_uuid) {
                    Ok(balance) => balance,
                    Err(e) => {
                        error!("Error getting balance: {}", e);
                        return Ok(0);
                    }
                };

                let amount_res = match args.get_value("amount") {
                    Arg::Num(amount) => amount,
                    _ => return Ok(0),
                };

                let amount = match amount_res {
                    Ok(Number::Float32(amount)) => amount,
                    _ => {
                        let error_msg = TextComponent::text("Invalid amount!");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        return Ok(0);
                    },
                };

                if amount > balance {
                    let error_msg = TextComponent::text("You cannot pay more than your balance!");
                    error_msg.color_named(NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(0);
                }

                if let Ok(frozen) = db.players.is_frozen(sender_uuid) {
                    if frozen {
                        let frozen_msg = TextComponent::text("You cannot pay while frozen!");
                        frozen_msg.color_named(NamedColor::DarkRed);
                        frozen_msg.bold(true);
                        sender.send_message(frozen_msg);
                        return Ok(0);
                    }
                };

                if let Ok(frozen) = db.players.is_frozen(target_uuid) {
                    if frozen {
                        let frozen_msg = TextComponent::text("That player is frozen!");
                        frozen_msg.color_named(NamedColor::DarkRed);
                        frozen_msg.bold(true);
                        sender.send_message(frozen_msg);
                        return Ok(0);
                    }
                }

                match db.players.remove_money(sender_uuid, amount) {
                    Ok(_) => (),
                    Err(e) => {
                        let error_msg = TextComponent::text("Error updating balance!");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        error!("Error updating balance: {}", e);
                        return Ok(0);
                    }
                }

                match db.players.add_money(target_uuid, amount) {
                    Ok(_) => (),
                    Err(e) => {
                        let error_msg = TextComponent::text("Error updating balance!");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        let _ = db.players.add_money(sender_uuid, amount);
                        error!("Error updating balance: {}", e);
                        return Ok(0);
                    }
                }

                let log = Transaction {
                    id: Uuid::now_v7(),
                    sender: sender_uuid,
                    receiver: target_uuid,
                    amount,
                    timestamp: Utc::now(),
                };

                match transaction_log.log_transaction(&log) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Error logging transaction: {}", e);
                        return Ok(0);
                    }
                };

                let msg = TextComponent::text(&format!("You have paid ${} to {}", format_money(amount), player.get_name()));
                msg.color_named(NamedColor::Green);
                sender.send_message(msg);

                let target_msg = TextComponent::text(&format!("You have received ${} from {}", format_money(amount), sender.as_player().unwrap().get_name()));
                target_msg.color_named(NamedColor::Green);

                match _server.get_player_by_uuid(player.get_id().as_str()) {
                    Some(target_player) => {
                        target_player.send_system_message(target_msg, true);
                    }
                    None => (),
                }



                return Ok(1);
            }
        }

        let error_msg = TextComponent::text("Missing arguments: 'Player' and 'Amount'");
        error_msg.color_named(NamedColor::DarkRed);
        error_msg.bold(true);
        sender.send_message(error_msg);

        Ok(1)
    }
}