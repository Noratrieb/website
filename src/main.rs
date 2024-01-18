mod submodule;
mod utils;

#[macro_use]
extern crate tracing;

use color_eyre::{eyre::Context, Result};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let sub_config =
        std::fs::read_to_string("submodules.toml").wrap_err("reading ./submodules.toml")?;
    let sub_config =
        submodule::Submodules::parse(&sub_config).wrap_err("invalid submodules.toml")?;
    submodule::sync(&sub_config).wrap_err("syncing subtrees")?;

    info!("Hello, world!");

    Ok(())
}
