//! Root index.html and some other static stuff

use std::path::Path;

use color_eyre::{eyre::WrapErr, Result};
use rand::seq::SliceRandom;

use crate::{utils, SlidesConfig};

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

    let mut context = tera::Context::new();

    context.insert("back_alley_name", back_alley_name.as_str());
    context.insert("talks", &config.talks);

    utils::copy_fn(&statics.join("root"), dist, |content, ext, opts| {
        if ext.is_some_and(|ext| matches!(ext, "html" | "css")) {
            if opts.dest_path.ends_with("back-alley.html") {
                opts.dest_path.set_file_name(&back_alley_name);
            }

            let content = String::from_utf8(content).wrap_err("HTML or CSS is invalid UTF-8")?;
            let mut tera = tera::Tera::default();
            tera.add_raw_template("template", &content)
                .wrap_err("parsing template")?;
            return tera
                .render("template", &context)
                .wrap_err("failed to render")
                .map(String::into_bytes);
        }

        Ok(content)
    })
    .wrap_err("copying root files")?;

    Ok(())
}
