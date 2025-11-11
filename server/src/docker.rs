use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use crate::globals::{PROJECT_DIR};
use crate::models::config::CONFIG_DIR;

pub const DOCKER_PREFIX: &str = include_str!("../../images/Dockerfile");
pub const DOCKER_SUFFIX: &str = include_str!("../../images/Dockerfile.cleanup");
pub const DOCKER_WIZARD: &str = include_str!("../../images/Dockerfile.wizard");
pub const DOCKER_IMAGE_PREFIX: &str = "robot-farm";
pub const DOCKER_IMAGE_WIZARD: &str = "robot-farm-wizard";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Worker,
    Wizard,
}

impl ImageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageType::Worker => "_worker",
            ImageType::Wizard => "_wizard"
        }
    }
}

/// Collection of generated schemas from main.rs
pub struct SchemaCollection<'a> {
    orchestrator: &'a str,
    worker: &'a str,
}

/// Creates 3 docker images, one for each Codex executor
pub fn make_worker_image(schemas: &SchemaCollection) {
    let p = PathBuf::from(format!("{}/Dockerfile", CONFIG_DIR.as_str()));
    let concatenated = combine_dockerfiles(p.to_str().unwrap());
    let p = PathBuf::from(PROJECT_DIR.as_str());
    let dir = fs::canonicalize(p).unwrap_or_else (|_| {panic!("project doesn't exist")});
    let proj_name =  dir.file_name().unwrap().to_str().unwrap();
    let worker_img = combine_image_name(proj_name, &ImageType::Worker);
    let tmp = TempDir::new().unwrap_or_else(|_| panic!("failed to create temporary directory"));
    // save generated schemas.orchestrator to {tmp}/schema.json
    /*
    $ docker build \
        --tag "{DOCKER_IMAGE_PREFIX}-orchestrator_{proj_name}" \
        --build-arg UID=1000 \
        --build-arg GID=1000 \
        -f - /{tmp}
     */ // << pipe {concatenated} to use as build script
    
    // save generated schemas.worker to {tmp}/schema.json
    /*
    $ docker build \
        --tag "{DOCKER_IMAGE_PREFIX}-worker_{proj_name}" \
        --build-arg UID=1000 \
        --build-arg GID=1000 \
        -f - /{tmp}
     */ // << pipe {concatenated} to use as build script
    
    /*
    $ docker build \
        --tag "{DOCKER_IMAGE_PREFIX}-orchestrator_{proj_name}" \
        --build-arg UID=1000 \
        --build-arg GID=1000 \
        -f - /{tmp}
     */ // << pipe {DOCKER_IMAGE_WIZARD} to use as build script

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
