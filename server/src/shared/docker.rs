use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::warn;

#[derive(Debug, Default, Clone)]
pub struct DockerRunBuilder {
    image: Option<String>,
    command: Option<Vec<String>>,
    remove_container: bool,
    interactive: bool,
    attach_streams: Vec<String>,
    user: Option<String>,
    workdir: Option<PathBuf>,
    volumes: Vec<(PathBuf, PathBuf, Option<String>)>,
    env_vars: Vec<(String, String)>,
}

impl DockerRunBuilder {
    pub fn new(image: impl Into<String>) -> Self {
        Self {
            image: Some(image.into()),
            ..Default::default()
        }
    }

    pub fn remove_container(mut self, enabled: bool) -> Self {
        self.remove_container = enabled;
        self
    }

    pub fn interactive(mut self, enabled: bool) -> Self {
        self.interactive = enabled;
        self
    }

    pub fn attach(mut self, stream: impl Into<String>) -> Self {
        self.attach_streams.push(stream.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn workdir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.workdir = Some(dir.into());
        self
    }

    pub fn volume(
        mut self,
        host: impl AsRef<Path>,
        container: impl AsRef<Path>,
        options: Option<String>,
    ) -> Self {
        self.volumes.push((
            host.as_ref().to_path_buf(),
            container.as_ref().to_path_buf(),
            options,
        ));
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.push((key.into(), value.into()));
        self
    }

    pub fn command<I, S>(mut self, command: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.command = Some(command.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn build(self) -> Vec<String> {
        let mut args = vec!["docker".to_string(), "run".to_string()];

        if self.interactive {
            args.push("-i".to_string());
        }

        if self.remove_container {
            args.push("--rm".to_string());
        }

        for stream in self.attach_streams {
            args.push("-a".to_string());
            args.push(stream);
        }

        if let Some(user) = self.user {
            args.push("-u".to_string());
            args.push(user);
        }

        if let Some(workdir) = self.workdir {
            args.push("--workdir".to_string());
            args.push(workdir.display().to_string());
        }

        for (host, container, opts) in self.volumes {
            let mut spec = format!("{}:{}", host.display(), container.display());
            if let Some(options) = opts {
                spec.push(':');
                spec.push_str(&options);
            }
            args.push("-v".to_string());
            args.push(spec);
        }

        for (key, value) in self.env_vars {
            args.push("-e".to_string());
            args.push(format!("{key}={value}"));
        }

        if let Some(image) = self.image {
            args.push(image);
        }

        if let Some(command) = self.command {
            args.extend(command);
        }

        args
    }
}

static DEFAULT_DOCKER_HOST: Lazy<String> = Lazy::new(resolve_default_docker_host);

/// Determine the MCP URL that containers should use when talking to the host
/// server. On macOS with OrbStack we must use `host.orb.internal`, otherwise
/// Docker's default hostname works.
pub fn ensure_default_mcp_url(port: u16) -> String {
    format!("http://{}:{port}/mcp", DEFAULT_DOCKER_HOST.as_str())
}

fn resolve_default_docker_host() -> String {
    if cfg!(target_os = "macos") {
        match Command::new("docker").arg("context").arg("show").output() {
            Ok(output) if output.status.success() => {
                let context = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if context == "orbstack" {
                    return "host.orb.internal".to_string();
                }
            }
            Ok(output) => {
                warn!(
                    status = ?output.status.code(),
                    "`docker context show` exited with non-zero status; using default hostname"
                );
            }
            Err(err) => {
                warn!(
                    ?err,
                    "failed to inspect docker context; using default hostname"
                );
            }
        }
    }

    "host.docker.internal".to_string()
}
