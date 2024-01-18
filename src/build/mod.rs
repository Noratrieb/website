//! This module assembles and builds the website.

mod blog;
mod slides;

use std::path::Path;

use color_eyre::{eyre::Context, Result};

use crate::Config;

pub fn assemble_website(config: &Config, submodules: &Path, dist: &Path) -> Result<()> {
    blog::build(&submodules.join("blog"), &dist.join("blog")).wrap_err("building blog")?;
    slides::build(
        &config.slides,
        &submodules.join("slides"),
        &dist.join("slides"),
    )
    .wrap_err("building slides")?;

    Ok(())
}
