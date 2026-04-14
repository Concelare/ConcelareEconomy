
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tracing::{info, warn};
use uuid::Uuid;

use crate::models::transaction::Transaction;

pub static TRANSACTIONS: OnceLock<Arc<TransactionService>> = OnceLock::new();

#[derive(Clone)]
pub struct TransactionService {
    file_path: PathBuf,
}

impl TransactionService {
    pub fn new(file_path: &str) -> Self {
        let instance = Self {
            file_path: PathBuf::from(file_path),
        };

        let _ = TRANSACTIONS.set(Arc::new(instance.clone()));
        instance
    }

    pub fn log_transaction(&self, transaction: &Transaction) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        let json = serde_json::to_string(transaction)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

        writeln!(file, "{json}")?;
        info!("Logged transaction {}", transaction.id);
        Ok(())
    }

    pub fn read_all(&self) -> std::io::Result<Vec<Transaction>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut transactions = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<Transaction>(&line) {
                Ok(transaction) => transactions.push(transaction),
                Err(err) => warn!("Failed to parse transaction line: {err}"),
            }
        }

        Ok(transactions)
    }

    pub fn search_by_id(&self, id: Uuid) -> std::io::Result<Option<Transaction>> {
        Ok(self.read_all()?.into_iter().find(|tx| tx.id == id))
    }

    pub fn search_by_sender(&self, sender: Uuid) -> std::io::Result<Vec<Transaction>> {
        Ok(self
            .read_all()?
            .into_iter()
            .filter(|tx| tx.sender == sender)
            .collect())
    }

    pub fn search_by_receiver(&self, receiver: Uuid) -> std::io::Result<Vec<Transaction>> {
        Ok(self
            .read_all()?
            .into_iter()
            .filter(|tx| tx.receiver == receiver)
            .collect())
    }

    pub fn search_by_amount(&self, amount: f32) -> std::io::Result<Vec<Transaction>> {
        Ok(self
            .read_all()?
            .into_iter()
            .filter(|tx| (tx.amount - amount).abs() < f32::EPSILON)
            .collect())
    }

    pub fn search_by_participant(&self, player: Uuid) -> std::io::Result<Vec<Transaction>> {
        Ok(self
            .read_all()?
            .into_iter()
            .filter(|tx| tx.sender == player || tx.receiver == player)
            .collect())
    }
}
