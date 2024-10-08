use std::any::Any;

use clap::{Parser, Subcommand};

mod budget;
use budget::{Budget, add_budget, remove_budget, add_transaction, edit_transaction, remove_transaction, view_budgets, get_sum};
use rusqlite::{Connection, Result};

static DB_NAME: &str = "budgets.db";

// simple program to add, edit, remove a budget
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A small program to manage your budgets",
    subcommand_value_name = "Budget Actions")]
struct MainCommands {
    /// optional budget subcommand
    #[command(subcommand)]
    budget_commands: Option<BudgetCommands>,
}

#[derive(Subcommand, Debug)]
enum BudgetCommands {
    AddBudget { name: String },
    EditBudget (EditBudgetCmd),
    RemoveBudget { name: String },
    ViewBudget { name: Option<String> },
    GetSum { name: Option<String> },
}
#[derive(Parser, Debug)]
#[command(
    long_about = "Edit your budget",
    subcommand_value_name = "Edit Budget Actions")]
struct EditBudgetCmd {
    /// name of budget,
    #[arg(long)]
    name: String,
    /// optional budget subcommand
    #[command(subcommand)]
    edit_budget_commands: Option<EditBudgetCommands>,
}
#[derive(Subcommand, Debug)]
enum EditBudgetCommands {
    AddTransaction(AddTransactionArgs),
    EditTransaction(EditTransactionArgs),
    RemoveTransaction(RemoveTransactionArgs),
}
#[derive(Parser, Debug)]
struct AddTransactionArgs {
    /// amount of transaction
    #[arg(long, allow_hyphen_values = true)]
    amount: f64,
    /// date of transaction
    #[arg(long)]
    date: String,
    /// description of transaction
    #[arg(long)]
    desc: Option<String>,
}
#[derive(Parser, Debug)]
struct EditTransactionArgs {
    /// id of transaction
    #[arg(long)]
    id: i32,
    /// date of transaction
    #[arg(long)]
    date: String,
    /// amount of transaction
    #[arg(long)]
    amount: f64,
    /// description of transaction
    #[arg(long)]
    desc: Option<String>,
}
#[derive(Parser, Debug)]
struct RemoveTransactionArgs {
    /// id of transaction
    #[arg(long)]
    id: i32,
}


// main function
fn main() {
    let main_cmd = MainCommands::parse();
    let conn = Connection::open(DB_NAME).unwrap();

    match &main_cmd.budget_commands {
        Some(BudgetCommands::AddBudget { name }) => {
            println!("Adding budget: {}", name);
            add_budget(&conn, &name.to_string().to_lowercase()).unwrap();
        }
        Some(BudgetCommands::RemoveBudget { name }) => {
            println!("Removing budget: {}", name);
            remove_budget(&conn, &name.to_string().to_lowercase()).unwrap();
        }
        Some(BudgetCommands::ViewBudget { name }) => {
            println!("Printing budget: {:?}", name);

            view_budgets(&conn, name.as_deref()).unwrap();
            //get_sum(&conn, Some(name.to_string().to_lowercase())).unwrap();
        }

        Some(BudgetCommands::GetSum { name }) => {
            println! ("Get sum of {:?}", name);
            get_sum(&conn, name.as_deref()).unwrap();
        }

        Some(BudgetCommands::EditBudget(edit_sub_cmd)) => {
            println!("Editing budget");
            match &edit_sub_cmd.edit_budget_commands {
                Some(EditBudgetCommands::AddTransaction(add_transaction_args)) => {
                    let desc = add_transaction_args.desc.as_ref().unwrap();
                    add_transaction(&conn
                                    , &edit_sub_cmd.name.clone().to_lowercase()
                                    , &add_transaction_args.date
                                    , &add_transaction_args.amount
                                    , &desc)
                                    .unwrap();
                }
                Some(EditBudgetCommands::EditTransaction(edit_transaction_args)) => {
                    println!("date transaction: {}", edit_transaction_args.date);
                    println!("amount transaction: {}", edit_transaction_args.amount);
                    println!("description transaction: {:?}", (edit_transaction_args.desc.clone()));
                    edit_transaction(&conn
                                    , &edit_sub_cmd.name
                                    , edit_transaction_args.id
                                    , edit_transaction_args.amount
                                    , &edit_transaction_args.desc
                                    , &edit_transaction_args.date)
                                    .unwrap();
                }
                Some(EditBudgetCommands::RemoveTransaction(remove_transaction_args)) => {
                    println!("Removing transaction: {}", remove_transaction_args.id);
                    remove_transaction(&conn, &edit_sub_cmd.name.to_lowercase(), &remove_transaction_args.id).unwrap();
                }
                None => {
                    println!("No command specified");
                }
            }
        }

        None => {
            println!("No command specified");
        }
    }

}
