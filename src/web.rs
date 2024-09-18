use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rusqlite::Connection;
use std::sync::Mutex;

mod budget;
use budget::{add_budget, remove_budget, view_budgets};

static DB_NAME: &str = "budgets.db";


#[derive(serde::Deserialize)]

struct BudgetInput {
    name: String,
}

#[derive(serde::Deserialize)]
struct EditBudgetInput {
    id : i32,
    description: Option<String>,
    amount: f64,
    date: String
}

struct AppState {
    conn: Mutex<Connection>,
}

async fn add_budget_api(data: web::Data<AppState>, budget: web::Json<BudgetInput>) -> impl Responder {
    let conn = data.conn.lock().unwrap();

    match add_budget(&conn, &budget.name) {
        Ok(_) => HttpResponse::Ok().json("Budget added successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open(DB_NAME).unwrap();

    let data = web::Data::new(AppState {
        conn: Mutex::new(conn),
    });

    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .route("/add_budget", web::post().to(add_budget_api))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}