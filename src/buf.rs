use std::path::Path;

use serde::Deserialize;

use crate::error::TonicBufBuildError;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct BufYaml {
    pub deps: Option<Vec<String>>,
}

impl BufYaml {
    pub(crate) fn load(file: &str) -> Result<BufYaml, TonicBufBuildError> {
        let f = std::fs::File::open(file)
            .map_err(|e| TonicBufBuildError::new(&format!("failed to read {}", file), e.into()))?;

        let buf: BufYaml = serde_yaml::from_reader(&f).map_err(|e| {
            TonicBufBuildError::new(&format!("failed to deserialize {}", file), e.into())
        })?;
        Ok(buf)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct BufWorkYaml {
    pub directories: Option<Vec<String>>,
}

impl BufWorkYaml {
    pub(crate) fn load(file: &str) -> Result<Self, TonicBufBuildError> {
        let buf_work_file = std::fs::File::open(file)
            .map_err(|e| TonicBufBuildError::new(&format!("failed to read {}", file), e.into()))?;

        let buf_work: BufWorkYaml = serde_yaml::from_reader(&buf_work_file).map_err(|e| {
            TonicBufBuildError::new(&format!("failed to deserialize {}", file), e.into())
        })?;

        Ok(buf_work)
    }
}

pub(crate) fn ls_files() -> Result<Vec<String>, TonicBufBuildError> {
    let child = std::process::Command::new("buf")
        .args(["ls-files"])
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
) -> Result<(), TonicBufBuildError> {
    if let Some(directories) = &buf_work.directories {
        for dir in directories {
            let buf = BufYaml::load(&format!("{}/buf.yaml", dir))?;
            export_all(&buf, export_dir)?;
        }
    }
    Ok(())
}
