use crate::manager::BudgetManager;
use core::panic;
use std::{fs::File, io::{BufReader, Write}};



pub struct FileManager {
    save_path: String
}

impl FileManager {
    pub fn new(path: String) -> FileManager {
        Self {
            save_path: path
        }
    }

    pub fn save(&self, budget_manager: &BudgetManager) {
        let serialized = serde_json::to_string(budget_manager).unwrap();
        let mut file = File::create(&self.save_path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    pub fn load(&self) -> BudgetManager {
        match File::open(&self.save_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let budget_manager = serde_json::from_reader(reader);
                match budget_manager {
                    Ok(budget_manager) => budget_manager,
                    Err(_) => {
                        println!("Error: Could not read file, it may be corrupted");
                        println!("Would you like to create a new file? (y/n)");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        if input.trim() == "y" {
                            BudgetManager::new()
                        } else {
                            panic!("Exiting program")
                        }
                    }
                }
            },
            Err(_) => BudgetManager::new()
        }
    }
}