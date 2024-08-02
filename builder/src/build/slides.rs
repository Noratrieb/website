//! Moving the slides from the reveal.js repo
//! The setup is currently a bit bad but I'm not sure what the best solution would look like.

use std::path::Path;

use color_eyre::{eyre::WrapErr, Result};

use crate::{utils, SlidesConfig};

pub fn build(config: &SlidesConfig, slides: &Path, dist: &Path) -> Result<()> {
    info!("Building slides");

    debug!("Copying reveal.js dist");

    utils::cp_r(&slides.join("dist"), &dist.join("dist")).wrap_err("copying reveal.js dist")?;
    utils::cp_r(&slides.join("plugin"), &dist.join("plugin")).wrap_err("copying reveal.js dist")?;

    for talk in &config.talks {
        let path = slides.join(talk.dir_name());
        let dist = dist.join(talk.dir_name());

        utils::cp_r(&path, &dist).wrap_err("copying slide data")?;
    }

    Ok(())
}
