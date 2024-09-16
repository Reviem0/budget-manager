use crate::transaction::{Transaction, TransactionType};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Budget {
    id: u32,
    name: String,
    budget: f32,
    transaction_list: HashMap<u32, Transaction>
}

impl Budget {
    pub fn new(new_name: &String, new_budget: f32, new_id: u32) -> Budget {
        Self {
            id: new_id,
            name: new_name.to_owned(),
            budget: new_budget,
            transaction_list: HashMap::new()
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_budget(&self) -> f32 {
        self.budget
    }

    pub fn get_transaction(&mut self, id: u32) -> Option<&mut Transaction>  {
        self.transaction_list.get_mut(&id)
    }

    pub fn update(&mut self, new_budget: f32) {
        self.budget = new_budget;
    }

    pub fn add_transaction(&mut self, t: Transaction) {
        let count: u32 = self.transaction_list.len().try_into().unwrap();
        self.transaction_list.insert(count, t);

    }

    pub fn display(&self) {
        println!("id: {} name: {} budget: {}", self.id, self.name, self.budget);
        println!("Transactions:");
        println!(
            "{0: <10} | {1: <10} | {2: <13} | {3: <10}",
            "id", "amount", "Type", "date", 
        );
        println!(
            "{0: <10} | {1: <10} | {2: <13} | {3: <10}",
            "-", "-", "-", "-"
        );

        // Collect transactions into a vector and sort by index
        let mut transactions: Vec<_> = self.transaction_list.iter().collect();
        transactions.sort_by_key(|&(index, _)| index);

        for (index, transaction) in transactions {
            match transaction.amount() {
                TransactionType::Transaction(amount) => {
                    println!("{0: <10} | {1: <10} | {2: <13} | {3: <10}",
                    index, amount, "Transaction", transaction.date().elapsed().unwrap().as_secs());
                }
                TransactionType::Deposit(amount) => {
                    println!("{0: <10} | {1: <10} | {2: <13} | {3: <10}",
                    index, amount, "Deposit", transaction.date().elapsed().unwrap().as_secs());
                }
            }
        }
    }

    pub fn calculate_total(&self) -> f32 {
        let mut total: f32 = self.budget;
        for (_, transaction) in self.transaction_list.iter() {
            match transaction.amount() {
                TransactionType::Transaction(amount) => {
                    total -= amount
                }
                TransactionType::Deposit(amount) => {
                    total+= amount
                }
            }
        }
        total
    }

    pub fn delete_transaction(&mut self, id: u32) {
        match self.transaction_list.get(&id) {
            Some(_) => {
                self.transaction_list.remove(&id);
            },
            None => println!("Invalid ID")
        }
    }

}