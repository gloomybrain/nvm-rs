use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use semver_rs::{Range, Version};
use serde_json;
use thiserror::Error;

use shared::local::{get_executable_path, list_local_versions};
use shared::package_json::PackageJson;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("no local versions found")]
    NoLocalVersionsFound(),

    #[error("no matching version found")]
    NoMatchingVersion(),
}

fn main() -> anyhow::Result<()> {
    let path = env::current_dir()?;

    let want = get_wanted_version(path);
    let available = list_local_versions().context("unable to read available versions")?;
    let best_match = find_best_match(want, available)?;

    run_executable(best_match)?;

    Ok(())
}

fn run_executable(version: Version) -> anyhow::Result<()> {
    let path = get_executable_path(version, "node".into())?;
    let args: Vec<String> = env::args().collect();

    Command::new(path).args(&args[1..]).spawn()?.wait()?;

    Ok(())
}

fn find_best_match(want: Option<Range>, available: Vec<Version>) -> Result<Version, LauncherError> {
    if available.is_empty() {
        return Err(LauncherError::NoLocalVersionsFound());
    }

    let mut current_best: Option<Version> = None;
    for version in available {
        current_best = match want {
            None => select(version, current_best),
            Some(ref range) => {
                if range.test(&version) {
                    select(version, current_best)
                } else {
                    current_best
                }
            }
        }
    }

    match current_best {
        Some(version) => Ok(version),
        None => Err(LauncherError::NoMatchingVersion()),
    }
}

fn select(version: Version, current_best: Option<Version>) -> Option<Version> {
    if let Some(best_yet) = current_best {
        if version > best_yet {
            Some(version)
        } else {
            Some(best_yet)
        }
    } else {
        Some(version)
    }
}

fn get_wanted_version(work_dir: PathBuf) -> Option<Range> {
    let mut path = work_dir;

    loop {
        if let Some(version) = read_node_version(&path) {
            return Some(version);
        }

        match path.parent() {
            None => {
                break;
            }
            Some(parent_path) => {
                let mut buf = PathBuf::new();
                buf.push(parent_path);

                path = buf;
            }
        }
    }

    None
}

fn read_node_version(path: &PathBuf) -> Option<Range> {
    if let Some(version) = read_package_json(path) {
        return Range::new(&version).parse().ok();
    }

    if let Some(version) = read_node_version_file(path) {
        return Range::new(&version).parse().ok();
    }

    if let Some(version) = read_nvmrc(path) {
        return Range::new(&version).parse().ok();
    }

    None
}

fn read_package_json(path: &PathBuf) -> Option<String> {
    let file_path = path.join("package.json");

    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(package_json) = serde_json::from_str::<PackageJson>(&content) {
            if let Some(engines) = package_json.engines {
                return engines.node;
            }
        }
    }

    None
}

fn read_node_version_file(path: &PathBuf) -> Option<String> {
    read_version_file(path, ".node-version")
}

fn read_nvmrc(path: &PathBuf) -> Option<String> {
    read_version_file(path, ".nvmrc")
}

fn read_version_file(path: &PathBuf, file_name: impl AsRef<Path>) -> Option<String> {
    let file_path = path.join(file_name);

    fs::read_to_string(file_path).ok()
}
