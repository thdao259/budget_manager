use rusqlite::{named_params, params, Connection, Result};
use clap::{Parser, Subcommand};

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
    PrintBudget { name: String },
    PrintBudgets,
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
    /// name of transaction
    #[arg(long)]
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
#[derive(Debug)]
struct Budget {
    id : i32,
    desc: String,
    date: String,
    amount: f64,
}

// function add_transaction
// add a transaction to the budget table
// function takes arguments as:
// name for the the table's name
// date for transaction date
// amount for transaction amount
// description for transaction description
fn add_transaction(name: String, date: String, amount: f64, description: String) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    create_table_if_not_exists(&conn, name.clone())?;
    conn.execute(
        format!("INSERT INTO {} (desc, date, amount) VALUES (?1, ?2, ?3)", name).as_str(),
        params![description, date, amount],
    )?;
    Ok(())
}

// function remove_transaction
// remove a transaction from the budget table
// function takes arguments as:
// name for the the table's name
// id for transaction id
fn remove_transaction(name: String, id: i32) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    conn.execute(
        format!("DELETE FROM {} WHERE id = ?1", name).as_str(),
        params![id],
    )?;
    Ok(())
}

// function edit_transaction
// edit a transaction from the budget table
// function takes arguments as:
// name for the the table's name
// id for transaction id
// date for transaction date
// amount for transaction amount
// description for transaction description

// create a table if not exists
// take connection and name as arguments
fn create_table_if_not_exists(conn: &Connection, name: String) -> Result<()> {
    conn.execute(
        format!("CREATE TABLE IF NOT EXISTS {} (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            desc TEXT,
            date TEXT NOT NULL,
            amount REAL NOT NULL)", name).as_str(),
        (),
    )?;
    Ok(())
}

// function add_budget
// create a budget with rusqlite
fn add_budget(name: String) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    create_table_if_not_exists(&conn, name.clone())?;
    Ok(())
}

// function remove_budget
// remove a table from the database with name as argument
fn remove_budget(name: String) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    conn.execute(
        format!("DROP TABLE IF EXISTS {}", name).as_str(),
        (),
    )?;
    Ok(())
}

// function to print a table
// get connection and name as input arguments
fn print_table(conn: &Connection, name: String) -> Result<()> {
    let mut stmt = conn.prepare(format!("SELECT * FROM {}", name).as_str())?;
    let budget_iter = stmt.query_map([], |row| {
        Ok(Budget {
            id : row.get(0)?,
            desc : row.get(1)?,
            date : row.get(2)?,
            amount : row.get(3)?,
        })
    })?;

    for budget in budget_iter {
        let budget = budget.unwrap();
        println!("id: {}, desc: {}, date: {}, amount: {}", budget.id, budget.desc, budget.date, budget.amount);
    }
    Ok(())
}


// function to print a budget
// arguments:
// name: for table name
fn print_budgets(name: Option<String>) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    match name {
        Some(name) => {
            print_table(&conn, name)?;
        }
        None => {
            let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name != 'sqlite_sequence'")?;
            let table_names: Vec<String> = stmt.query_map([], |row| row.get(0))?
                .filter_map(Result::ok)
                .collect();

            for table in table_names {
                println!("Print budget: {}", table);
                print_table(&conn, table)?;
            }
        }
    }
    Ok(())
}

fn print_sum_table(conn: &Connection, name: String) -> Result<()> {
    //let conn = Connection::open(DB_NAME)?;
    let total: f64 = conn.query_row(format!("SELECT COALESCE(SUM(amount),0) FROM {}", name).as_str(), [], |row| row.get(0))?;

    println!("Total sum for budget {} = {}", name, total);
    Ok(())
}

fn print_sum(name: Option<String>) -> Result<()>{
    let conn = Connection::open(DB_NAME)?;

    match name {
        Some(name) => {
            print_sum_table (&conn, name)?;
        }
        None => {
            let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name != 'sqlite_sequence'")?;
            let tables_name: Vec<String> = stmt.query_map([], |row| row.get(0))?
                                                .filter_map(Result::ok)
                                                .collect();
            for table in tables_name {
                print_sum_table(&conn, table)?;
            }
        }
    }
    Ok(())
}
// main function
fn main() {
    let main_cmd = MainCommands::parse();

    match &main_cmd.budget_commands {
        Some(BudgetCommands::AddBudget { name }) => {
            println!("Adding budget: {}", name);
            add_budget(name.to_string().to_lowercase()).unwrap();
        }
        Some(BudgetCommands::RemoveBudget { name }) => {
            println!("Removing budget: {}", name);
            remove_budget(name.to_string().to_lowercase()).unwrap();
        }
        Some(BudgetCommands::PrintBudget { name }) => {
            println!("Printing budget: {}", name);
            print_budgets(Some(name.to_string().to_lowercase())).unwrap();
            print_sum(Some(name.to_string().to_lowercase())).unwrap();
        }
        Some(BudgetCommands::PrintBudgets {}) => {
            println!("PRINT ALL");
            print_budgets(None).unwrap();
            print_sum(None).unwrap();
        }
        Some(BudgetCommands::EditBudget(edit_sub_cmd)) => {
            println!("Editing budget");
            match &edit_sub_cmd.edit_budget_commands {
                Some(EditBudgetCommands::AddTransaction(add_transaction_args)) => {
                    add_transaction(edit_sub_cmd.name.clone().to_lowercase(), add_transaction_args.date.clone(), add_transaction_args.amount, add_transaction_args.desc.clone().unwrap_or_default()).unwrap();
                }
                Some(EditBudgetCommands::EditTransaction(edit_transaction_args)) => {
                    println!("date transaction: {}", edit_transaction_args.date);
                    println!("amount transaction: {}", edit_transaction_args.amount);
                    println!("description transaction: {:?}", Some(edit_transaction_args.desc.clone()));
                }
                Some(EditBudgetCommands::RemoveTransaction(remove_transaction_args)) => {
                    println!("Removing transaction: {}", remove_transaction_args.id);
                    remove_transaction(edit_sub_cmd.name.clone().to_lowercase(), remove_transaction_args.id).unwrap();
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
