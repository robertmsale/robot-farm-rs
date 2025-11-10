use crate::globals::{PROJECT_DIR};

pub const DOCKER_PREFIX: &str = include_str!("../../images/Dockerfile");
pub const DOCKER_SUFFIX: &str = include_str!("../../images/Dockerfile.cleanup");
pub const DOCKER_WIZARD: &str = include_str!("../../images/Dockerfile.wizard");
pub const DOCKER_IMAGE_PREFIX: &str = "robot-farm_";
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

pub fn make_worker_image() -> String {
    //
}

pub fn combine_dockerfiles(paths: &[&str]) -> String {
    use std::{fs, path::Path};

    let mut sections = Vec::with_capacity(paths.len() + 2);
    sections.push(DOCKER_PREFIX.trim_end().to_string());

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_dir() {
            panic!("Dockerfile segment '{path}' is a directory, expected a file.");
        }

        let content = fs::read_to_string(path_obj).unwrap_or_else(|err| {
            panic!("Failed to read Dockerfile segment '{path}': {err}");
        });

        sections.push(content.trim().to_string());
    }

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
