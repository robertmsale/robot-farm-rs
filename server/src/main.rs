use axum::Router;
use axum::serve;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[path = "routes/lib.rs"]
mod routes;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let app: Router = routes::build_routes();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!("Robot Farm API listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;
    Ok(())
}
