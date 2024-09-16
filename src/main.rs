mod args;
mod budget;
mod manager;
mod transaction;
mod file_manager;

use args::CLIArgs;
use clap::Parser;
use file_manager::FileManager;


fn main() {
    let args = CLIArgs::parse();

    let file_manager = FileManager::new("budget.json".to_string());
    let mut budgetmanager = file_manager.load();

    match &args.entity_type {
        args::EntityType::Budget(command) => budgetmanager.budget_commands(command),

        args::EntityType::Transaction(command) => budgetmanager.transaction_commands(command),
    }

    file_manager.save(&budgetmanager);
}