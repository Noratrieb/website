//! Implements a simplistic form of git submodules.
//!
//! The config format is as follows:
//!
//! A list of
//! ```toml
//! [[submodule]]
//! name = ""
//! url = ""
//! commit = ""
//! ```
//!
//! For example,
//! ```toml
//! [[submodule]]
//! name = "nixos"
//! url = "https://github.com/Noratrieb/nixos.git"
//! commit = "c5b2fc10b9266b105d792d958b8f13479866a7bd"
//! ```
//!
//! This module will check them out into a directory called `submodules` in the current directory.
//! Make sure to put this directory into `.gitignore`.

use std::{path::Path, process};

use color_eyre::{
    eyre::{Context, OptionExt},
    Result,
};

use crate::utils;

pub struct Submodules {
    configs: Vec<SyncConfig>,
}

pub struct SyncConfig {
    name: String,
    url: String,
    commit: String,
}

impl Submodules {
    pub fn parse(s: &str) -> Result<Submodules> {
        let doc = s.parse::<toml::Table>().wrap_err("invalid toml")?;
        let subs = doc
            .get("submodule")
            .ok_or_eyre("no top-level submodule tables")?;
        let mods = subs.as_array().ok_or_eyre("submodule is not an array")?;

        let mut configs = Vec::new();

        for module in mods {
            let map = module.as_table().ok_or_eyre("submodule is not a table")?;

            let get_str = |name| -> Result<String> {
                Ok(map
                    .get(name)
                    .ok_or_eyre(format!("{name} is missing"))?
                    .as_str()
                    .ok_or_eyre(format!("{name} is not a string"))?
                    .into())
            };

            configs.push(SyncConfig {
                name: get_str("name")?,
                url: get_str("url")?,
                commit: get_str("commit")?,
            });
        }

        Ok(Self { configs })
    }
}

pub fn sync(path: &Path, config: &Submodules) -> color_eyre::Result<()> {
    info!("Syncing submodules...");

    utils::create_dir_if_not_exist(path)?;

    for sync in &config.configs {
        let name = &sync.name;
        let url = sync.url.as_str();

        let span = info_span!("Syncing submodule", ?name, ?url);
        let _span = span.enter();

        let sub_path = path.join(name);
        if !sub_path.exists() {
            info!(?name, ?url, "Cloning");
            let mut cmd = process::Command::new("git");
            cmd.args(["clone", url, sub_path.to_str().unwrap()]);
            utils::run_process(&mut cmd)?;
        } else {
            debug!(?name, ?url, "Repo already exists");
        }

        let current_commit = utils::run_process(
            process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&sub_path),
        )
        .wrap_err("running git rev-parse HEAD")?;

        debug!(?current_commit, "Current commit");

        if current_commit.trim() != sync.commit {
            info!("Need to change commit");
            let commit_exists = utils::run_process(
                process::Command::new("git")
                    .args(["cat-file", "-t", sync.commit.as_str()])
                    .current_dir(&sub_path),
            );
            if !commit_exists.is_ok_and(|typ| typ == *"commit\n") {
                info!("Must fetch commit");
                utils::run_process(process::Command::new("git").current_dir(&sub_path).args([
                    "fetch",
                    "origin",
                    sync.commit.as_str(),
                ]))?;
            }

            utils::run_process(process::Command::new("git").current_dir(&sub_path).args([
                "reset",
                "--hard",
                sync.commit.as_str(),
            ]))?;
        }
    }

    Ok(())
}
