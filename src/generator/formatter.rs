use std::{path::Path, process::Command};

pub struct Formatter<'a>(&'a str);

impl<'a> Formatter<'a> {
    pub fn new(name: &'a str) -> Self {

        Self(name)
    }

    pub fn format(&self) {

        let manifest = Path::new(self.0);

        let absolute_path = manifest
            .canonicalize()
            .expect("Failed to get absolute path of the manifest");

        tracing::info!("Formatting code in {}...", absolute_path.display());

        Command::new("cargo")
            .current_dir(&absolute_path)
            .arg("fmt")
            .output()
            .expect("Failed to format the code");

        tracing::info!("Code formatted successfully");

        tracing::info!("Running clippy for linting and fixing...");

        let output = Command::new("cargo")
            .current_dir(&absolute_path)
            .arg("clippy")
            .arg("--fix")
            .arg("--allow-dirty")
            .arg("--allow-no-vcs")
            .output()
            .expect("Failed to run clippy");

        if output.status.success() {
            tracing::info!("Clippy fixes completed successfully");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!(
                "Clippy encountered issues:\n{}",
                stderr
            );
        }
    }
}
