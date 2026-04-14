use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, Number};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use tracing::error;
use uuid::Uuid;
use crate::services::database;
use crate::util::numbers::format_money;

pub fn give_money_command(cmd: &Command){
    let give_node =  CommandNode::literal("give").execute(GiveMoneyCommandExecutor);
    let player_arg = ArgumentType::Players;
    let arg = ArgumentType::Float((Some(0f32), Some(f32::MAX)));

    give_node.then({
        let node = CommandNode::argument("player", &player_arg);
        node.then(CommandNode::argument("amount", &arg).execute(GiveMoneyCommandExecutor));
        node
    });

    cmd.then(give_node);
}

struct GiveMoneyCommandExecutor;

impl CommandHandler for GiveMoneyCommandExecutor {
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
                    let frozen_msg = TextComponent::text("This player is frozen!");
                    frozen_msg.color_named(NamedColor::DarkRed);
                    frozen_msg.bold(true);
                    sender.send_message(frozen_msg);
                    return Ok(1);
                }

                let amount_res = match args.get_value("amount") {
                    Arg::Num(amount) => amount,
                    _ => return Ok(0),
                };

                let amount = match amount_res {
                    Ok(Number::Float32(amount)) => amount,
                    _ => return Ok(0),
                };

                match db.players.add_money(Uuid::from_str(player.get_id().as_str()).unwrap(), amount) {
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

                let msg = TextComponent::text(&format!("You have given ${} to {}", format_money(amount), player.get_name()));
                msg.color_named(NamedColor::Green);
                msg.bold(true);
                sender.send_message(msg);
            }

            return Ok(1);
        }

        let message = TextComponent::text("Missing arguments: 'Player' and 'Amount'");
        message.color_named(NamedColor::DarkRed);
        message.bold(true);
        sender.send_message(message);
        Ok(1)
    }
}