use crate::services::transaction::TRANSACTIONS;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use std::str::FromStr;
use uuid::Uuid;

pub fn revert_command(cmd: &Command) {
    let node = CommandNode::literal("revert").execute(RevertCommandExecutor);
    node.then(CommandNode::argument("transaction", &ArgumentType::String(StringType::SingleWord)).execute(RevertCommandExecutor));

    cmd.then(node);
}

pub struct RevertCommandExecutor;

impl CommandHandler for RevertCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let transactions = TRANSACTIONS.get().unwrap();

        if let Arg::Simple(arg) = args.get_value("transaction") {
            let transaction_id = Uuid::from_str(arg.as_str()).unwrap();
            let transaction = match transactions.search_by_id(transaction_id) {
                Ok(Some(transaction)) => transaction,
                Ok(None) => {
                    let error_msg = pumpkin_plugin_api::text::TextComponent::text("Transaction not found!");
                    error_msg.color_named(pumpkin_plugin_api::common::NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(0);
                },
                Err(_) => {
                    let error_msg = pumpkin_plugin_api::text::TextComponent::text("Error searching transaction");
                    error_msg.color_named(pumpkin_plugin_api::common::NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(0);
                },
            };

            return match transactions.revert_transaction(transaction.id) {
                Ok(_) => {

                    let msg = pumpkin_plugin_api::text::TextComponent::text("Transaction reverted successfully!");
                    msg.color_named(pumpkin_plugin_api::common::NamedColor::Green);
                    msg.bold(true);
                    sender.send_message(msg);
                    Ok(0)
                },
                Err(_) => {
                    let error_msg = pumpkin_plugin_api::text::TextComponent::text("Error reverting transaction");
                    error_msg.color_named(pumpkin_plugin_api::common::NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    Ok(0)
                },
            };

        }

        let error_msg = pumpkin_plugin_api::text::TextComponent::text("Missing argument: 'transaction'");
        error_msg.color_named(pumpkin_plugin_api::common::NamedColor::DarkRed);
        error_msg.bold(true);
        sender.send_message(error_msg);

        Ok(0)
    }
}