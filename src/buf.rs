use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::TonicBufBuildError;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct BufYaml {
    pub deps: Option<Vec<String>>,
}

impl BufYaml {
    pub(crate) fn load(file: &Path) -> Result<BufYaml, TonicBufBuildError> {
        let f = std::fs::File::open(file).map_err(|e| {
            TonicBufBuildError::new(&format!("failed to read {:?}", file.as_os_str()), e.into())
        })?;

        let buf: BufYaml = serde_yaml::from_reader(&f).map_err(|e| {
            TonicBufBuildError::new(
                &format!("failed to deserialize {:?}", file.as_os_str()),
                e.into(),
            )
        })?;
        Ok(buf)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct BufWorkYaml {
    pub directories: Option<Vec<String>>,
}

impl BufWorkYaml {
    pub(crate) fn load(file: &Path) -> Result<Self, TonicBufBuildError> {
        let buf_work_file = std::fs::File::open(file).map_err(|e| {
            TonicBufBuildError::new(&format!("failed to read {:?}", file.as_os_str()), e.into())
        })?;

        let buf_work: BufWorkYaml = serde_yaml::from_reader(&buf_work_file).map_err(|e| {
            TonicBufBuildError::new(
                &format!("failed to deserialize {:?}", file.as_os_str()),
                e.into(),
            )
        })?;

        Ok(buf_work)
    }
}

pub(crate) fn ls_files(proto_path: &Path) -> Result<Vec<String>, TonicBufBuildError> {
    let child = std::process::Command::new("buf")
        .args([OsStr::new("ls-files"), proto_path.as_os_str()])
        .output()
        .map_err(|e| TonicBufBuildError::new("failed to execute `buf ls-files'", e.into()))?;

    if !child.status.success() {
        return Err(TonicBufBuildError::new_without_cause(&format!(
            "failed to execute `buf ls-files', returned status code {}: {}",
            child.status.code().unwrap_or(-1),
            std::str::from_utf8(&child.stderr).unwrap()
        )));
    }
    let protos = std::str::from_utf8(&child.stdout)
        .map_err(|e| TonicBufBuildError::new("failed to decode `buf ls-files' output", e.into()))?
        .trim_end()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok(protos)
}

pub(crate) fn export_all(buf: &BufYaml, export_dir: &Path) -> Result<(), TonicBufBuildError> {
    let export_dir = export_dir.to_str().unwrap();

    if let Some(deps) = &buf.deps {
        for dep in deps {
            std::process::Command::new("buf")
                .args(["export", dep, "-o", export_dir])
                .spawn()
                .map_err(|e| {
                    TonicBufBuildError::new(
                        &format!("failed to execute `buf export {} -o {}'", &dep, &export_dir),
                        e.into(),
                    )
                })?
                .wait()
                .map_err(|e| {
                    TonicBufBuildError::new(
                        &format!("failed to execute `buf export {} -o {}'", &dep, &export_dir),
                        e.into(),
                    )
                })?;
        }
    }

    Ok(())
}

pub(crate) fn export_all_from_workspace(
    buf_work: &BufWorkYaml,
    export_dir: &Path,
    workspace_dir: &Path,
) -> Result<Vec<PathBuf>, TonicBufBuildError> {
    let mut buf_dirs = vec![];
    if let Some(directories) = &buf_work.directories {
        for dir in directories {
            let mut buf_dir = PathBuf::from(workspace_dir);
            buf_dir.push(dir);
            buf_dirs.push(buf_dir.clone());
            buf_dir.push("buf.yaml");

            let buf = BufYaml::load(buf_dir.as_path())?;

            export_all(&buf, export_dir)?;
        }
    }
    Ok(buf_dirs)
}
