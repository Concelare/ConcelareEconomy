use std::path::PathBuf;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use pumpkin_plugin_api::events::EventPriority;
use tracing::*;
use crate::commands::register_commands;
use crate::events::on_join::OnJoinEvent;

mod events;
mod models;
mod services;
mod repositories;
mod commands;
pub mod util;

const PERMISSION_BASE: &str = "ConcelareEconomy:command.";

struct EconomyPlugin;
impl Plugin for EconomyPlugin {
    fn new() -> Self {
        EconomyPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "ConcelareEconomy".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Concelare".into()],
            description: "Concelare Economy plugin for managing player balances and transactions.".into(),
        }
    }

    fn on_load(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Loading Concelare Economy plugin...");

        let db_path = PathBuf::from(_context.get_data_folder()).join("economy.db");
        let log_path = PathBuf::from(_context.get_data_folder()).join("economy.log");
        if !db_path.exists() {
            let _ = std::fs::File::create(&db_path);
            info!("Created economy database at {}", db_path.to_str().unwrap());
        }

        if !log_path.exists() {
            let _ = std::fs::File::create(&log_path);
            info!("Created economy transaction log at {}", log_path.to_str().unwrap());
        }

        let _db = services::database::DatabaseService::new(db_path.to_str().unwrap())?;
        info!("Connected to economy database");
        let _transaction_service = services::transaction::TransactionService::new(log_path.to_str().unwrap());
        info!("Initialized transaction service");
        _context.register_event_handler(OnJoinEvent { db: _db.clone() }, EventPriority::Highest, false)?;
        info!("Registered OnJoinEvent handler");
        register_commands(&_context);
        info!("Completed Loading Concelare Economy plugin.");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Concelare Economy plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(EconomyPlugin);