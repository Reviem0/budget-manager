#![allow(unused)]
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::manager::{self, BudgetManager};

#[derive(Serialize, Deserialize)]
pub enum TransactionType {
    Transaction(f32),
    Deposit(f32)
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    budget_id: u32,
    transaction_id: u32,
    amount: TransactionType,
    date: SystemTime
}

impl Transaction {
    pub fn new(new_id: u32, new_amount: TransactionType) -> Transaction {
        Self {
            budget_id: new_id,
            transaction_id: 0,
            amount: new_amount,
            date: SystemTime::now()
        }
    }

    pub fn transaction_id(&self) -> u32 {
        self.transaction_id
    }

    pub fn amount(&self) -> &TransactionType {
        &self.amount
    }

    pub fn date(&self) -> SystemTime {
        self.date
    }

    pub fn update(&mut self, new_amount: f32) {
        match self.amount {
            TransactionType::Transaction(_) => {
                self.amount = TransactionType::Transaction(new_amount)
            }

            TransactionType::Deposit(_) => {
                self.amount = TransactionType::Deposit(new_amount)
            }
        }
    }
}