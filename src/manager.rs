use crate::{
    args::{self, BudgetCommand, TransactionCommand}, 
    transaction::{Transaction, TransactionType},
    budget::Budget
};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BudgetManager {
    budget_list: HashMap<u32, Budget>,
    count: u32
}

impl BudgetManager {
    pub fn new() -> BudgetManager {
        Self {
            budget_list: HashMap::new(),
            count: 0
        }
    }

    pub fn get_budget(&mut self, id: u32) -> Option<&mut Budget> {
        self.budget_list.get_mut(&id)
    }


    fn create_budget(&mut self, new_name: &String, new_budget: f32) {
        if self.budget_list.contains_key(&self.count) {
            self.budget_list.iter().count();
        }
        self.budget_list.insert(self.count, Budget::new(new_name, new_budget, self.count));
        self.count += 1;  
    }

        fn list_budgets(&self) {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "id", "name", "budget", "%Δ"
        );
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "-", "-", "-", "-"
        );
    
        // Collect budgets into a vector and sort by index
        let mut budgets: Vec<_> = self.budget_list.iter().collect();
        budgets.sort_by_key(|&(index, _)| index);
    
        for (_, budget) in budgets {
            let total = budget.calculate_total();
            println!(
                "{0: <10} | {1: <10} | {2: <10} | {3: <10}", 
                budget.get_id(), 
                budget.get_name(), 
                total, 
                ((total / budget.get_budget()) * 100.0).round() as i32
            );
        }
    }

    fn update_budget(&mut self, id: u32, new_budget: f32) {
        match self.budget_list.get_mut(&id) {
            Some(budget) => budget.update(new_budget),
            None => println!("Invalid id {}", id)
        }
    }

    fn delete_budget(&mut self, id:u32) {
        match self.budget_list.get_mut(&id) {
            Some(_) => {
                self.budget_list.remove(&id);
            }, 
            None => println!("Invalid id {}", id)
        }
    }

    pub fn add_transaction(&mut self, id:u32, transaction: Transaction) {
        match self.budget_list.get_mut(&id) {
            Some(budget) => budget.add_transaction(transaction),
            None => println!("Erorr: Invalid ID"),
        }
    }

    pub fn display_budget(&self, id: u32) {
        match self.budget_list.get(&id) {
            Some(budget) => budget.display(),
            None => println!("Error: Invalid ID"),
        }
    }

    pub fn create_transaction(&mut self, id: u32, amount: f32) {
        let transaction = Transaction::new(id, TransactionType::Transaction(amount));
        self.add_transaction(id, transaction);
    }

    pub fn place_deposit(&mut self, id: u32, amount: f32) {
        let transaction = Transaction::new(id, TransactionType::Deposit(amount));
        self.add_transaction(id, transaction);
    }

    pub fn update_transaction(&mut self, id: u32, transaction_id: u32, new_amount: f32) {
        match self.get_budget(id) {
            Some(budget) => {
                match budget.get_transaction(transaction_id) {
                    Some(transaction) => transaction.update(new_amount),
                    None => println!("Invalid transaction id {}", transaction_id)
                }
            },
            None => println!("Invalid budget id {}", id)
        }
    }

    pub fn delete_transaction(&mut self, id:u32, transaction_id:u32) {
        match self.get_budget(id) {
            Some(budget) => {
                budget.delete_transaction(transaction_id)
            },
            None => println!("Invalid budget id {}", id)
        }
    }
    

    pub fn budget_commands(&mut self,budget_command: &BudgetCommand ) {
        match &budget_command.command {
            args::BudgetSubCommand::Create(command) => {
                println!("Creating Budget with name {} and budget {}", command.budget_name, command.budget_amount);
                self.create_budget(&command.budget_name, command.budget_amount);
                self.list_budgets();
            }
            args::BudgetSubCommand::Update(command) => {
                self.update_budget(command.budget_id, command.budget_amount)
            },
            args::BudgetSubCommand::Delete(command) => {
                self.delete_budget(command.budget_id)
            },
            args::BudgetSubCommand::Display(command) => {
                self.display_budget(command.budget_id)
            },
            args::BudgetSubCommand::List => self.list_budgets(),
        }
    }

    pub fn transaction_commands(&mut self, transaction_command: &TransactionCommand) {
        match &transaction_command.command {
            args::TransactionSubCommand::Create(command) => {
                self.create_transaction(command.budget_id, command.amount);
            }

            args::TransactionSubCommand::Update(command) => {
                self.update_transaction(command.budget_id, command.transaction_id, command.amount)
            },
            args::TransactionSubCommand::Delete(command) => self.delete_transaction(command.budget_id, command.transaction_id),
            args::TransactionSubCommand::Deposit(command) => {
                self.place_deposit(command.budget_id ,command.amount)
            },
        }
    }
}