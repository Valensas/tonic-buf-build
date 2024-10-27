//! tonic-buf-build allows you to integrate [buf.build](https://buf.build) with [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
//! Using buf.build and tonic, you can easily manage third party dependencies for proto files and generate code for your proto files in Rust.
//! Works with both [buf.yaml](https://buf.build/docs/configuration/v1/buf-yaml) and [buf.work.yaml](https://buf.build/docs/configuration/v1/buf-work-yaml).
//!
//!
//! ## Usage
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! tonic-buf-build = "*"
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
//! To use buf workspaces, you simply call `tonic-buf-build::compile_from_buf_workspace` instead.
//!
//! For complete and working examples, take a look at the examples folder.
//!

use error::TonicBufBuildError;
use scopeguard::defer;
use std::path::{Path, PathBuf};

mod buf;
pub mod error;

fn tempdir() -> PathBuf {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push(uuid::Uuid::new_v4().to_string());
    temp_dir
}

#[derive(Default)]
pub struct TonicBufConfig<P: AsRef<Path> = &'static str> {
    pub buf_dir: Option<P>,
}

pub fn compile_from_buf_workspace(
    tonic_builder: tonic_build::Builder,
    config: Option<prost_build::Config>,
) -> Result<(), TonicBufBuildError> {
    compile_from_buf_workspace_with_config::<&'static str>(
        tonic_builder,
        config,
        TonicBufConfig::default(),
    )
}

pub fn compile_from_buf_workspace_with_config<P: AsRef<Path>>(
    tonic_builder: tonic_build::Builder,
    config: Option<prost_build::Config>,
    tonic_buf_config: TonicBufConfig<P>,
) -> Result<(), TonicBufBuildError> {
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }
    let buf_dir = tonic_buf_config
        .buf_dir
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or(".".as_ref());
    let mut buf_work_file = PathBuf::from(buf_dir);
    buf_work_file.push("buf.work.yaml");
    let buf_work = buf::BufWorkYaml::load(buf_work_file.as_path())?;

    let buf_work_directories = buf::export_all_from_workspace(&buf_work, &export_dir, buf_dir)?;
    let mut includes = vec![export_dir.clone()];

    for dep in buf_work_directories {
        includes.push(dep);
    }

    let protos = buf::ls_files(buf_dir)?;

    match config {
        None => tonic_builder.compile_protos(&protos, &includes),
        Some(config) => tonic_builder.compile_protos_with_config(config, &protos, &includes),
    }
    .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}

pub fn compile_from_buf(
    tonic_builder: tonic_build::Builder,
    config: Option<prost_build::Config>,
) -> Result<(), TonicBufBuildError> {
    compile_from_buf_with_config::<&'static str>(tonic_builder, config, TonicBufConfig::default())
}

pub fn compile_from_buf_with_config<P: AsRef<Path>>(
    tonic_builder: tonic_build::Builder,
    config: Option<prost_build::Config>,
    tonic_buf_config: TonicBufConfig<P>,
) -> Result<(), TonicBufBuildError> {
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }
    let buf_dir = tonic_buf_config
        .buf_dir
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or(".".as_ref());
    let mut buf_file = PathBuf::from(buf_dir);
    buf_file.push("buf.yaml");
    let buf = buf::BufYaml::load(buf_file.as_path())?;

    buf::export_all(&buf, &export_dir)?;
    let protos = buf::ls_files(buf_dir)?;
    let includes = [buf_dir, &export_dir];

    match config {
        None => tonic_builder.compile_protos(&protos, &includes),
        Some(config) => tonic_builder.compile_protos_with_config(config, &protos, &includes),
    }
    .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}
