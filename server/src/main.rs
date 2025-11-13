use crate::{docker::make_worker_image, globals::PROJECT_DIR, shared::git as shared_git};
use anyhow::{Context, anyhow};
use axum::Router;
use axum::serve;
use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[path = "ai/lib.rs"]
mod ai;
mod compliance;
mod db;
mod docker;
mod globals;
mod models;
#[path = "routes/lib.rs"]
mod routes;
mod shared;
mod system;
#[path = "threads/lib.rs"]
mod threads;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let cli = parse_cli_args()?;
    if let Some(workspace) = cli.workspace.as_ref() {
        env::set_current_dir(workspace)
            .with_context(|| format!("failed to set workspace to {}", workspace.display()))?;
        info!("workspace set to {}", workspace.display());
    }

    routes::config::ensure_config_exists()
        .map_err(|err| anyhow!("failed to initialize config: {err}"))?;

    let staging = Path::new(PROJECT_DIR.as_str()).join("staging");
    shared_git::ensure_non_bare_repo(&staging)
        .unwrap_or_else(|err| panic!("staging repository check failed: {err}"));

    let _db_pool = db::ensure_db()
        .await
        .expect("failed to initialize database");

    compliance::init();
    system::init_system_state();

    make_worker_image();
    threads::init_background_threads();
    let app: Router = routes::build_routes();
    let port = 8080;
    let ipv6_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), port);
    let ipv4_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

    let listener = match TcpListener::bind(ipv6_addr).await {
        Ok(listener) => {
            info!("Robot Farm API listening on [::]:{port}");
            listener
        }
        Err(err) => {
            warn!(
                error = %err,
                "IPv6 bind failed, falling back to IPv4 interface",
            );
            let listener = TcpListener::bind(ipv4_addr).await?;
            info!("Robot Farm API listening on 0.0.0.0:{port}");
            listener
        }
    };
    serve(listener, app).await?;
    Ok(())
}

#[derive(Default)]
struct CliArgs {
    workspace: Option<PathBuf>,
}

fn parse_cli_args() -> Result<CliArgs, anyhow::Error> {
    let mut args = env::args().skip(1);
    let mut cli = CliArgs::default();

    while let Some(arg) = args.next() {
        if arg == "--workspace" {
            let value = args
                .next()
                .ok_or_else(|| anyhow!("--workspace requires a path argument"))?;
            cli.workspace = Some(PathBuf::from(value));
        } else if let Some(rest) = arg.strip_prefix("--workspace=") {
            if rest.is_empty() {
                return Err(anyhow!("--workspace=PATH requires a non-empty path"));
            }
            cli.workspace = Some(PathBuf::from(rest));
        } else {
            return Err(anyhow!("unknown argument: {arg}"));
        }
    }

    Ok(cli)
}
