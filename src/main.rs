mod auth;
mod config;
mod db;
mod graphql;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let cfg = config::env::AppConfig::from_env();
    let pool = db::pool::create_pool(&cfg.database_url).await;

    db::smoke::smoke_test(&pool).await;

    let app = routes::router(pool, cfg.jwt_secret);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server failed");
}
