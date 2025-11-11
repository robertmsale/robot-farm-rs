use crate::docker::make_worker_image;
use anyhow::{Context, anyhow};
use axum::Router;
use axum::serve;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing::info;

#[path = "ai/lib.rs"]
mod ai;
mod db;
mod docker;
mod globals;
#[path = "routes/lib.rs"]
mod routes;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let cli = parse_cli_args()?;
    if let Some(workspace) = cli.workspace.as_ref() {
        env::set_current_dir(workspace)
            .with_context(|| format!("failed to set workspace to {}", workspace.display()))?;
        info!("workspace set to {}", workspace.display());
    }

    routes::config::ensure_config_exists()
        .map_err(|err| anyhow!("failed to initialize config: {err}"))?;

    make_worker_image();
    let app: Router = routes::build_routes();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!("Robot Farm API listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
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
