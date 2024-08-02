//! Builds my blog, built with hugo.

use std::{path::Path, process::Command};

use color_eyre::{eyre::Context, Result};

use crate::utils;

pub fn build(blog: &Path, dist: &Path) -> Result<()> {
    info!("Building blog with hugo");

    // Patch config
    let config =
        std::fs::read_to_string(blog.join("config.toml")).wrap_err("reading blog config")?;
    let config = config.replace("baseURL = \"/\"", "baseURL = \"/blog/\"");
    std::fs::write(blog.join("config.toml"), config).wrap_err("writing patched config.toml")?;

    utils::run_process(
        Command::new("hugo")
            .args(["--minify", "--destination", dist.to_str().unwrap()])
            .current_dir(blog),
    )?;

    Ok(())
}
