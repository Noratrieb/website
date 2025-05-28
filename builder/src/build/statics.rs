//! Root index.html and some other static stuff

use std::path::Path;

use color_eyre::{eyre::WrapErr, Result};

use crate::{utils, SlidesConfig};

pub fn build(
    _rng: &mut rand::rngs::StdRng,
    config: &SlidesConfig,
    statics: &Path,
    dist: &Path,
) -> Result<()> {
    let mut context = tera::Context::new();

    context.insert("talks", &config.talks);

    utils::copy_fn(&statics.join("root"), dist, |content, ext, _opts| {
        if ext.is_some_and(|ext| matches!(ext, "html" | "css")) {
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
