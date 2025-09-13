mod db;
mod models;
mod routes;
mod handlers;
mod entities;

use axum::{
    routing::get,
    Router
};
use routes::create_routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    println!("Iniciando conexión a la base de datos...");
    let pool = db::init_pool().await.unwrap();
    println!("Conexión a la base de datos establecida correctamente");
    
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", create_routes())
        .with_state(pool);

    

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor escuchando en {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

