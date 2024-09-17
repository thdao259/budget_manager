use rusqlite::{named_params, params, Connection, Result};

#[derive(Debug)]
pub struct Budget {
    id : i32,
    desc: String,
    date: String,
    amount: f64,
}

// create a table if not exists
// take connection and name as arguments
pub fn add_budget(conn: &Connection, name: &String) -> Result<()> {
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

// function remove_budget
// remove a table from the database with name as argument
pub fn remove_budget(conn: &Connection, name: &String) -> Result<()> {
    conn.execute(
        format!("DROP TABLE IF EXISTS {}", name).as_str(),
        (),
    )?;
    Ok(())
}

fn is_table_exist(conn: &Connection, name: &String) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT count(*) FROM sqlite_master where type='table' and name=?1",)?;

    let exist:i32 = stmt.query_row([name], |row| row.get(0))?;

    Ok(exist > 0)
}
// function add_transaction
// add a transaction to the budget table
// function takes arguments as:
// name for the the table's name
// date for transaction date
// amount for transaction amount
// description for transaction description
pub fn add_transaction(conn: &Connection, name: &String, date: &String, amount: &f64, description: &String) -> Result<()> {
    if !is_table_exist(conn, name)? {
        let _ = add_budget(conn, name);
    }
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
pub fn remove_transaction(conn: &Connection, name: &String, id: &i32) -> Result<()> {
    conn.execute(
        format!("DELETE FROM {} WHERE id = ?1", name).as_str(),
        params![id],
    )?;
    Ok(())
}

pub fn edit_transaction(conn: &Connection
                        , name: &str
                        , id: i32
                        , amount: f64
                        , desc: &Option<String>
                        , date: &str) -> Result<()> {

    //println!("TEST 1");
    let description = desc.as_deref().unwrap_or_default();
    let query = format!("REPLACE INTO {} (id, desc, date, amount) VALUES (?1, ?2, ?3, ?4)", &name);
    //println!("TEST 2: {}", &description);
    conn.execute(&query, params![id, description, date, amount])?;

    Ok(())
}
// function to print a table
// get connection and name as input arguments
fn view_budget(conn: &Connection, name: &str) -> Result<()> {
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
pub fn view_budgets(conn: &Connection, name: Option<&str>) -> Result<()> {
    match name {
        Some(name) => {
            view_budget(&conn, &name)?;
        }
        None => {
            let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name != 'sqlite_sequence'")?;
            let table_names: Vec<String> = stmt.query_map([], |row| row.get(0))?
                .filter_map(Result::ok)
                .collect();

            for table in table_names {
                println!("Print budget: {}", table);
                view_budget(&conn, &table)?;
            }
        }
    }
    Ok(())
}

fn print_sum_table(conn: &Connection, name: &str) -> Result<()> {
    let total: f64 = conn.query_row(format!("SELECT COALESCE(SUM(amount),0) FROM {}", name).as_str(), [], |row| row.get(0))?;

    println!("Total sum for budget {} = {}", name, total);
    Ok(())
}

pub fn get_sum(conn: &Connection, name: Option<&str>) -> Result<()>{
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
                print_sum_table(&conn, table.as_str())?;
            }
        }
    }
    Ok(())
}