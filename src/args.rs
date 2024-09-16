use clap::{
    Args, Parser, Subcommand
};

#[derive(Parser,Debug)]
#[clap(author, version, about)]
pub struct CLIArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Add, remove, edit, display a budget
    Budget(BudgetCommand),

    // Add, remove, edit transactions associated with a budget
    Transaction(TransactionCommand)
}

#[derive(Debug, Args)]
pub struct BudgetCommand {
    #[clap(subcommand)]
    pub command: BudgetSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum BudgetSubCommand {
    /// Create a Budget
    Create(CreateBudget),

    /// Update a Budget
    Update(UpdateBudget),

    /// Delete a Budget
    Delete(DeleteBudget),

    /// Display a Budget
    Display(DisplayBudget),

    /// List all budgets
    List,
}

#[derive(Debug, Args)]
pub struct CreateBudget {
    /// Name of the Budget
    pub budget_name: String,

    /// Total Amount in Budget
    pub budget_amount: f32
}

#[derive(Debug, Args)]
pub struct UpdateBudget{
    /// Name of the Budget
    pub budget_id: u32,

    /// Total Amount in Budget
    pub budget_amount: f32
}

#[derive(Debug, Args)]
pub struct DeleteBudget{
    /// Name of the Budget
    pub budget_id: u32,
}

#[derive(Debug, Args)]
pub struct DisplayBudget{
    /// Name of the Budget
    pub budget_id: u32,
}

#[derive(Debug, Args)]
pub struct TransactionCommand {
    #[clap(subcommand)]
    pub command: TransactionSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum TransactionSubCommand{
    /// Create a Transaction
    Create(CreateTransaction),

    /// Deposit Amount
    Deposit(DepositTransaction),

    /// Update a Transaction
    Update(UpdateTransaction),

    /// Delete a Transaction
    Delete(DeleteTransaction),

}

#[derive(Debug, Args)]
pub struct CreateTransaction {
    /// Name of the Budget
    pub budget_id: u32,

    /// Total Amount in Budget
    pub amount: f32
}

#[derive(Debug, Args)]
pub struct DepositTransaction {
    /// Name of the Budget
    pub budget_id: u32,

    /// Total Amount of the Transaction
    pub amount: f32
}

#[derive(Debug, Args)]
pub struct UpdateTransaction{
    /// Name of the Budget
    pub budget_id: u32,

    /// Unique ID of the Transaction
    pub transaction_id: u32,

    // Total Amount of the Transaction
    pub amount: f32
    
}

#[derive(Debug, Args)]
pub struct DeleteTransaction{
    /// Name of the Budget
    pub budget_id: u32,

    /// Unique ID of the Transaction
    pub transaction_id: u32
}
