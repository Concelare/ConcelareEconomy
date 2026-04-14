use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::error;
use uuid::Uuid;
use crate::services::database;
use crate::util::numbers::format_money;

pub fn balance_command() -> Command {
    let names = ["balance".to_string(), "bal".to_string()];
    let description = "Check your balance";

    let cmd = Command::new(&names, description);
    cmd.then(CommandNode::argument("player", &ArgumentType::Players).execute(BalanceCommandExecutor));
    cmd.execute(BalanceCommandExecutor)
}

struct BalanceCommandExecutor;

impl CommandHandler for BalanceCommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let db = database::get_database();

        if let Arg::Players(players) = args.get_value("player") {
           for player in players {

               let balance = match db.players.get_balance(Uuid::from_str(player.get_id().as_str()).unwrap()) {
                   Ok(balance) => balance,
                   Err(e) => {
                       error!("Error getting balance: {}", e);
                       return Ok(0);
                   }
               };

               let frozen = match db.players.is_frozen(Uuid::from_str(player.get_id().as_str()).unwrap()) {
                   Ok(frozen) => frozen,
                   Err(e) => {
                       error!("Error checking if player is frozen: {}", e);
                       return Ok(0);
                   }
               };

               let message = TextComponent::text(format!("{}'s Balance: $", player.get_name()).as_str());
               message.color_named(NamedColor::Green);
               message.bold(true);

               let msg_part2 = TextComponent::text(format_money(balance).as_str());
               msg_part2.color_named(NamedColor::White);
               msg_part2.bold(false);

               message.add_child(msg_part2);

               sender.send_message(message);

               if frozen {
                   let frozen_msg = TextComponent::text("Notice: This player's account is frozen!");
                   frozen_msg.color_named(NamedColor::DarkRed);
                   frozen_msg.bold(true);
                   sender.send_message(frozen_msg);
               }

               return Ok(1);
           }
        }

        let balance = match db.players.get_balance(Uuid::from_str(sender.as_player().unwrap().get_id().as_str()).unwrap()) {
            Ok(balance) => balance,
            Err(e) => {
                error!("Error getting player balance: {}", e);
                return Ok(0);
            }
        };

        let frozen = match db.players.is_frozen(Uuid::from_str(sender.as_player().unwrap().get_id().as_str()).unwrap()) {
            Ok(frozen) => frozen,
            Err(e) => {
                error!("Error checking if player is frozen: {}", e);
                return Ok(0);
            }
        };

        let message = TextComponent::text("Your Balance: $".to_string().as_str());
        message.color_named(NamedColor::Green);
        message.bold(true);

        let msg_part2 = TextComponent::text(format_money(balance).as_str());
        msg_part2.color_named(NamedColor::White);
        msg_part2.bold(false);


        message.add_child(msg_part2);

        sender.send_message(message);

        if frozen {
            let frozen_msg = TextComponent::text("Notice: Your balance is frozen!");
            frozen_msg.color_named(NamedColor::DarkRed);
            frozen_msg.bold(true);
            sender.send_message(frozen_msg);
        }
        
        Ok(1)
    }
}