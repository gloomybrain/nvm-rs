use semver_rs::Version;
use std::env;
use std::fs::{canonicalize, read_dir};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalError {
    #[error("unable to resolve local path: {0}")]
    UnableToResolve(#[from(io::Error)] PathBuf),

    #[error("unable to read directory: {0}")]
    UnableToReadDir(#[from(io::Error)] PathBuf),
}

pub fn get_nvm_dir() -> Result<PathBuf, LocalError> {
    let path = match env::var("NVM_DIR") {
        Ok(path) => path,
        Err(_) => String::from("~/.nvm"),
    };

    let path_buf = PathBuf::from(path);

    canonicalize(&path_buf).map_err(|_| LocalError::UnableToResolve(path_buf))
}

pub fn get_executable_path(
    version: Version,
    executable_name: String,
) -> Result<PathBuf, LocalError> {
    let mut path_buf = get_nvm_dir()?;
    path_buf.push("versions");
    path_buf.push("node");
    path_buf.push(format!("v{}", version.to_string()));
    path_buf.push("bin");
    path_buf.push(executable_name);

    canonicalize(&path_buf).map_err(|_| LocalError::UnableToResolve(path_buf))
}

pub fn list_local_versions() -> Result<Vec<Version>, LocalError> {
    let mut versions_dir = get_nvm_dir()?;
    versions_dir.push("versions");
    versions_dir.push("node");

    let file_names = match read_dir(&versions_dir) {
        Ok(files) => files
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter_map(|tag| Version::new(&tag).parse().ok())
            .collect(),
        Err(_) => {
            return Err(LocalError::UnableToReadDir(versions_dir));
        }
    };

    Ok(file_names)
}
