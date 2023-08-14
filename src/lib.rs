//! tonic_buf_build allows you to integrate [buf.build](https://buf.build) with [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
//! Using buf.build and tonic, you can easily manage third party dependencies for proto files and generate code for your proto files in Rust.
//! Works with both [buf.yaml](https://buf.build/docs/configuration/v1/buf-yaml) and [buf.work.yaml](https://buf.build/docs/configuration/v1/buf-work-yaml).
//!
//!
//! ## Usage
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! tonic_buf_build = {path = "../../"}
//! tonic-build = "*"
//! ```
//!
//! Then, in your build.rs:
//!
//! ```rust
//! fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
//!    tonic_buf_build::compile_from_buf(tonic_build::configure(), None)?;
//!    Ok(())
//! }
//! ```
//!
//! To use buf workspaces, you simply call `tonic_buf_build::compile_from_buf_workspace` instead.
//!
//! For complete and working examples, take a look at the examples folder.
//!

use error::TonicBufBuildError;
use scopeguard::defer;

mod buf;
pub mod error;

fn tempdir() -> std::path::PathBuf {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push(uuid::Uuid::new_v4().to_string());
    temp_dir
}

pub fn compile_from_buf_workspace(
    tonic_builder: tonic_build::Builder, config: Option<prost_build::Config>,
) -> Result<(), TonicBufBuildError> {
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }

    let buf_work = buf::BufWorkYaml::load("buf.work.yaml")?;

    buf::export_all_from_workspace(&buf_work, &export_dir)?;
    let buf_work_directories = buf_work.directories.unwrap_or_default();
    let mut includes = vec![export_dir.to_str().unwrap().to_string()];

    for dep in buf_work_directories {
        includes.push(dep);
    }

    let protos = buf::ls_files()?;

    match config {
        None => tonic_builder.compile(&protos, &includes),
        Some(config) => tonic_builder.compile_with_config(config, &protos, &includes),
    }
    .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}

pub fn compile_from_buf(
    tonic_builder: tonic_build::Builder, config: Option<prost_build::Config>,
) -> Result<(), TonicBufBuildError> {
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }
    let buf = buf::BufYaml::load("buf.yaml")?;

    buf::export_all(&buf, &export_dir)?;
    let protos = buf::ls_files()?;
    let includes = [".", export_dir.to_str().unwrap()];

    match config {
        None => tonic_builder.compile(&protos, &includes),
        Some(config) => tonic_builder.compile_with_config(config, &protos, &includes),
    }
    .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}
