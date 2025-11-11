use crate::ai::schemas::{OrchestratorTurn, WorkerTurn, generated_schema_for};
use crate::globals::PROJECT_DIR;
use crate::models::config::CONFIG_DIR;
use schemars::JsonSchema;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::info;

pub const DOCKER_PREFIX: &str = include_str!("../../images/Dockerfile");
pub const DOCKER_SUFFIX: &str = include_str!("../../images/Dockerfile.cleanup");
pub const DOCKER_WIZARD: &str = include_str!("../../images/Dockerfile.wizard");
pub const DOCKER_IMAGE_PREFIX: &str = "robot-farm-rs";
pub const DOCKER_IMAGE_WIZARD: &str = "robot-farm-rs-wizard";

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
pub fn make_worker_image() {
    let p = PathBuf::from(format!("{}/Dockerfile", CONFIG_DIR.as_str()));
    let concatenated = combine_dockerfiles(p.to_str().unwrap());
    let p = PathBuf::from(PROJECT_DIR.as_str());
    let dir = fs::canonicalize(p).unwrap_or_else(|_| panic!("project doesn't exist"));
    let proj_name = dir.file_name().unwrap().to_str().unwrap();
    let worker_img = combine_image_name(proj_name, &ImageType::Worker);
    let tmp = TempDir::new().unwrap_or_else(|_| panic!("failed to create temporary directory"));
    info!("Creating Orchestrator image");
    generate_response_schema::<OrchestratorTurn>(&tmp);
    run_docker_build(
        tmp.path(),
        &concatenated,
        &format!("{DOCKER_IMAGE_PREFIX}-orchestrator_{proj_name}"),
    );
    info!("Creating Worker image");
    generate_response_schema::<WorkerTurn>(&tmp);
    run_docker_build(
        tmp.path(),
        &concatenated,
        &format!("{DOCKER_IMAGE_PREFIX}-worker_{proj_name}"),
    );
    info!("Creating Wizard image");
    run_docker_build(tmp.path(), DOCKER_WIZARD, DOCKER_IMAGE_WIZARD);
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
    use crate::ai::schemas::{OrchestratorTurn, WorkerTurn, generated_schema_for};

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
