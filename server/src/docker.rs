use crate::ai::schemas::{OrchestratorTurn, WorkerTurn, generated_schema_for};
use crate::db::image_cache;
use crate::globals::PROJECT_NAME;
use crate::routes::config::CONFIG_DIR;
use schemars::JsonSchema;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::LazyLock;
use tempfile::TempDir;
use tracing::{debug, info};

pub const DOCKER_PREFIX: &str = include_str!("../../images/Dockerfile");
pub const DOCKER_SUFFIX: &str = include_str!("../../images/Dockerfile.cleanup");
pub const DOCKER_WIZARD: &str = include_str!("../../images/Dockerfile.wizard");
pub const DOCKER_IMAGE_PREFIX: &str = "robot-farm-rs";
pub const DOCKER_IMAGE_WIZARD: &str = "robot-farm-rs-wizard";
pub static DOCKER_IMAGE_WORKER: LazyLock<String> = LazyLock::new(|| {
    let proj_name = PROJECT_NAME.as_str();
    format!("{DOCKER_IMAGE_PREFIX}-worker_{proj_name}")
});
pub static DOCKER_IMAGE_ORCHESTRATOR: LazyLock<String> = LazyLock::new(|| {
    let proj_name = PROJECT_NAME.as_str();
    format!("{DOCKER_IMAGE_PREFIX}-orchestrator_{proj_name}")
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Worker,
    Wizard,
}

impl ImageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageType::Worker => "_worker",
            ImageType::Wizard => "_wizard",
        }
    }
}

/// Creates 3 docker images, one for each Codex executor
pub async fn make_worker_image() {
    let p = PathBuf::from(format!("{}/Dockerfile", CONFIG_DIR.as_str()));
    if !p.exists() {
        if let Some(parent) = p.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::File::create(&p).unwrap_or_else(|err| {
            panic!(
                "failed to create Dockerfile fragment at {}: {}",
                p.display(),
                err
            );
        });
    }
    let concatenated = combine_dockerfiles(p.to_str().unwrap());
    let orch_image = DOCKER_IMAGE_ORCHESTRATOR.as_str();
    let work_image = DOCKER_IMAGE_WORKER.as_str();
    let tmp = TempDir::new().unwrap_or_else(|_| panic!("failed to create temporary directory"));
    let concatenated_hash = hash_content(&concatenated);
    let wizard_hash = hash_content(DOCKER_WIZARD);

    if should_build_image(orch_image, &concatenated_hash).await {
        info!("Creating Orchestrator image");
        generate_response_schema::<OrchestratorTurn>(&tmp);
        run_docker_build(tmp.path(), &concatenated, orch_image);
        store_hash(orch_image, &concatenated_hash).await;
    } else {
        info!("Orchestrator image up-to-date; skipping build");
    }
    if should_build_image(work_image, &concatenated_hash).await {
        info!("Creating Worker image");
        generate_response_schema::<WorkerTurn>(&tmp);
        run_docker_build(tmp.path(), &concatenated, work_image);
        store_hash(work_image, &concatenated_hash).await;
    } else {
        info!("Worker image up-to-date; skipping build");
    }
    if should_build_image(DOCKER_IMAGE_WIZARD, &wizard_hash).await {
        info!("Creating Wizard image");
        run_docker_build(tmp.path(), DOCKER_WIZARD, DOCKER_IMAGE_WIZARD);
        store_hash(DOCKER_IMAGE_WIZARD, &wizard_hash).await;
    } else {
        info!("Wizard image up-to-date; skipping build");
    }
}
fn write_schema_file(tmp: &TempDir, schema: schemars::Schema) -> Result<(), anyhow::Error> {
    let bytes = serde_json::to_vec_pretty(&schema)?;

    let path = tmp.path().join("schema.json");
    match fs::exists(&path) {
        Ok(false) => {
            fs::File::create(&path).unwrap_or_else(|_| panic!("failed to write schema file"));
        }
        Err(e) => panic!("Failed to write schema into temp folder: {}", e),
        _ => {}
    }
    fs::write(&path, bytes).unwrap_or_else(|_| panic!("failed to create schema file in temp dir"));
    Ok(())
}
fn generate_response_schema<T: JsonSchema>(tmp: &TempDir) {
    write_schema_file(&tmp, generated_schema_for::<T>())
        .unwrap_or_else(|_| panic!("failed to write schema file"));
}

fn run_docker_build(tmp_dir: &std::path::Path, dockerfile: &str, tag: &str) {
    use std::process::{Command, Stdio};

    let mut child = Command::new("docker")
        .args([
            "build",
            "--tag",
            tag,
            "--build-arg",
            "UID=1000",
            "--build-arg",
            "GID=1000",
            "-f",
            "-",
            tmp_dir.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|_| panic!("failed to spawn docker build process"));

    {
        let stdin = child.stdin.as_mut().expect("docker build missing stdin");
        stdin
            .write_all(dockerfile.as_bytes())
            .expect("failed to write dockerfile to stdin");
    }

    let status = child.wait().expect("failed to wait on docker build");
    if !status.success() {
        panic!("docker build for tag {tag} failed with {status}");
    }
}

pub fn combine_dockerfiles(path: &str) -> String {
    use std::{fs, path::Path};

    let mut sections = Vec::with_capacity(path.len() + 2);
    sections.push(DOCKER_PREFIX.trim_end().to_string());

    let path_obj = Path::new(path);
    if path_obj.is_dir() {
        panic!("Dockerfile segment '{path}' is a directory, expected a file.");
    }

    let content = fs::read_to_string(path_obj).unwrap_or_else(|err| {
        panic!("Failed to read Dockerfile segment '{path}': {err}");
    });

    sections.push(content.trim().to_string());

    sections.push(DOCKER_SUFFIX.trim_start().to_string());
    sections.join("\n")
}

pub fn combine_image_name(workspace_name: &str, image_type: &ImageType) -> String {
    format!(
        "{}{}{}",
        DOCKER_IMAGE_PREFIX,
        workspace_name,
        image_type.as_str()
    )
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn docker_image_exists(tag: &str) -> bool {
    Command::new("docker")
        .args(["image", "inspect", tag])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

async fn should_build_image(tag: &str, hash: &str) -> bool {
    let cached = image_cache::get_hash(tag).await.ok().flatten();
    let exists = docker_image_exists(tag);
    match cached {
        Some(stored) if stored == hash && exists => {
            debug!(tag, "image matches cached hash; build skipped");
            false
        }
        _ => true,
    }
}

async fn store_hash(tag: &str, hash: &str) {
    if let Err(err) = image_cache::upsert_hash(tag, hash).await {
        debug!(%err, tag, "failed to record image hash");
    }
}
