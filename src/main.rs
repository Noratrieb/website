mod build;
mod submodule;
mod utils;

#[macro_use]
extern crate tracing;

use std::path::Path;

use color_eyre::{eyre::Context, Result};
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

const ROOT_DIR: &str = env!("ROOT_DIR");

#[derive(Deserialize)]
struct Config {
    slides: SlidesConfig,
}

#[derive(Deserialize)]
struct SlidesConfig {
    talks: Vec<String>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let root = Path::new(ROOT_DIR);

    // Set the current dir to nonsense to fail everything that relies on it
    let _ = std::env::set_current_dir("/");

    if std::env::args().nth(1).as_deref() == Some("clean") {
        info!("Cleaning dist");
        match std::fs::remove_dir_all(root.join("dist")) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            e => return e.wrap_err("removing dist"),
        }
        return Ok(());
    }

    let config =
        std::fs::read_to_string(root.join("config.toml")).wrap_err("reading config.toml")?;
    let config = toml::from_str::<Config>(&config).wrap_err("parsing config.toml")?;

    let sub_config = std::fs::read_to_string(root.join("submodules.toml"))
        .wrap_err("reading submodules.toml")?;
    let sub_config =
        submodule::Submodules::parse(&sub_config).wrap_err("invalid submodules.toml")?;
    let submodules_path = root.join("submodules");
    submodule::sync(&submodules_path, &sub_config).wrap_err("syncing subtrees")?;

    let dist_path = root.join("dist");
    build::assemble_website(&config, &root.join("static"), &submodules_path, &dist_path)?;

    Ok(())
}
