use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use tracing::{error, info};
use crate::PERMISSION_BASE;

pub mod balance;
pub mod give_money;
pub mod freeze_account;
pub mod unfreeze_account;
pub mod remove_money;
pub mod set_money;
pub mod pay;
pub mod log;

pub fn register_commands(context: &Context) {
    info!("Registering Commands...");

    info!("Registering balance command permission...");
    let balance_permission = Permission {
        node: PERMISSION_BASE.to_string() + "balance",
        description: "Allows users to check their balance".to_string(),
        default: PermissionDefault::Allow,
        children: Vec::new(),
    };
    match context.register_permission(&balance_permission) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register balance permission: {}", e);
            return;
        }
    }
    info!("Registered balance command permission");
    info!("Registering balance command...");
    context.register_command(balance::balance_command(), balance_permission.node.as_str());
    info!("Registered balance command");

    info!("Registering pay command permission...");
    let pay_permission = Permission {
        node: PERMISSION_BASE.to_string() + "pay",
        description: "Allow users to pay other players".to_string(),
        default: PermissionDefault::Allow,
        children: vec![],
    };
    match context.register_permission(&pay_permission) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register pay permission: {}", e);
            return;
        }
    }
    info!("Registered pay command permission");
    info!("Registering pay command...");
    context.register_command(pay::pay_command(), pay_permission.node.as_str());
    info!("Registered pay command");

    info!("Registering Economy Admin commands...");
    let names = ["economy".to_string(), "eco".to_string()];
    let cmd = Command::new(&names, "ConcelareEconomy Plugin for Pumpkin");

    give_money::give_money_command(&cmd);
    freeze_account::freeze_command(&cmd);
    unfreeze_account::unfreeze_command(&cmd);
    remove_money::remove_money_command(&cmd);
    set_money::set_money_command(&cmd);
    log::log_command(&cmd);

    let permission = Permission {
        node: PERMISSION_BASE.to_string() + "Admin",
        description: "ConcelareEconomy Admin Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };

    match context.register_permission(&permission) {
        Ok(_) => (),
        Err(e) => error!("Error registering permission: {}", e),
    };

    context.register_command(cmd, permission.node.as_str());
    info!("Registered Economy Admin commands");
    info!("Registered Commands!");
}