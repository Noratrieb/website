//! Root index.html and some other static stuff

use std::{fs, path::Path};

use askama::Template;
use color_eyre::{eyre::WrapErr, Result};
use rand::seq::SliceRandom;

use crate::{utils, SlidesConfig, Talk};

#[derive(askama::Template)]
#[template(path = "slides.html")]
struct Slides {
    talks: Vec<Talk>,
}

pub fn build(
    rng: &mut rand::rngs::StdRng,
    config: &SlidesConfig,
    statics: &Path,
    dist: &Path,
) -> Result<()> {
    let back_alley_name = b"abcdefghijklmnopqrstuvwxyz"
        .choose_multiple(rng, 6)
        .map(|&c| char::from_u32(c.into()).unwrap())
        .collect::<String>();

    let back_alley_name = format!("back-alley-{back_alley_name}.html");

    utils::cp_content(&statics.join("root"), dist).wrap_err("copying root files")?;

    let back_alley = dist.join("back-alley.html");
    std::fs::copy(&back_alley, dist.join(&back_alley_name)).wrap_err("copying back-alley.html")?;
    fs::remove_file(back_alley).wrap_err("deleting normal back-alley.html")?;

    let index_html = dist.join("index.html");
    let index = fs::read_to_string(&index_html).wrap_err("reading index.html")?;
    fs::write(
        index_html,
        index.replace("back-alley.html", &back_alley_name),
    )
    .wrap_err("writing back index.html")?;

    let slide_html = Slides {
        talks: config.talks.clone(),
    }
    .render()
    .wrap_err("rendering slide index")?;

    fs::write(dist.join("slides").join("index.html"), slide_html)
        .wrap_err("writing slides index.html")?;

    Ok(())
}
