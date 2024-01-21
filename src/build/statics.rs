//! Root index.html and some other static stuff

use std::{fs, path::Path};

use askama::Template;
use color_eyre::{eyre::WrapErr, Result};

use crate::{utils, SlidesConfig, Talk};

#[derive(askama::Template)]
#[template(path = "slides.html")]
struct Slides {
    talks: Vec<Talk>,
}

pub fn build(config: &SlidesConfig, statics: &Path, dist: &Path) -> Result<()> {
    utils::cp_content(&statics.join("root"), dist).wrap_err("copying root files")?;

    let slide_html = Slides {
        talks: config.talks.clone(),
    }
    .render()
    .wrap_err("rendering slide index")?;

    fs::write(dist.join("slides").join("index.html"), slide_html)
        .wrap_err("writing slides index.html")?;

    Ok(())
}
