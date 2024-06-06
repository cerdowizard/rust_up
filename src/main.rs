use axum::{response::{self, IntoResponse},routing::{self, get},Json,Router};
use std::sync::Arc;
use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
async fn heath_checker_handler() -> impl IntoResponse {
    const MESSAGE:&str= "heath_checker handler";

    let json_response = serde_json::json!(
        {
            "code": 200,
            "status": "OK",
            "message": MESSAGE
        }
    );
    Json(json_response)
}

pub struct AppState {
    db:Pool<Postgres>
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = match PgPoolOptions::new() 
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        } 
    };
    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = Router::new().route("/", get(heath_checker_handler));
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
