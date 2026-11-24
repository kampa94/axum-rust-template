use backend::{
    config::Config,
    repository::{post_repository::PostRepository, user_repository::UserRepository},
    routes,
    services::{post_service::PostService, user_service::UserService},
    state::AppState,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{EnvFilter, fmt};
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let config = Config::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("connessione al DB fallita");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrazioni fallite");
    tracing::info!("database connected and migrations applied");

    let user_service = UserService::new(UserRepository::new(pool.clone()));
    let post_service = PostService::new(PostRepository::new(pool));
    let state = AppState::new(config, user_service, post_service);
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
