mod build;
mod submodule;
mod utils;

#[macro_use]
extern crate tracing;

use std::{
    path::Path,
    process::{self, Stdio},
    time,
};

use color_eyre::{
    eyre::{bail, eyre, Context},
    Result,
};
use notify::{RecursiveMode, Watcher};
use serde::Deserialize;
use tracing::level_filters::LevelFilter;
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
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let root = Path::new(ROOT_DIR);

    // Set the current dir to nonsense to fail everything that relies on it
    let _ = std::env::set_current_dir("/");

    match std::env::args().nth(1).as_deref() {
        Some("clean") => {
            info!("Cleaning dist");
            match std::fs::remove_dir_all(root.join("dist")) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                e => return e.wrap_err("removing dist"),
            }
            Ok(())
        }
        Some("watch") => watch(root),
        Some("build") => build(root),
        Some(cmd) => bail!("invalid subcommand {cmd}"),
        None => bail!("no subcommand provided"),
    }
}

fn watch(root: &'static Path) -> Result<()> {
    let (send, recv) = std::sync::mpsc::sync_channel(1);
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(_) => {
            let _ = send.send(());
        }
        Err(e) => {
            eprintln!("watch error: {e:?}");
        }
    })
    .wrap_err("creating watcher")?;

    watcher.watch(&root.join("static"), RecursiveMode::Recursive)?;
    watcher.watch(&root.join("config.toml"), RecursiveMode::NonRecursive)?;

    info!("Starting webserver");
    std::thread::spawn(move || {
        let run = || -> Result<()> {
            let path = root.join("dist");
            let mut server = process::Command::new("live-server");
            server
                .current_dir(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null());

            let mut child = server.spawn().wrap_err("failed to spawn `live-server`.\
                Install https://github.com/tapio/live-server into your PATH, for example with nix, see shell.nix")?;
            let exit = child.wait().wrap_err("interrupt waiting for live-server")?;
            bail!("live-server exited early, exit: {exit}");
        };

        if let Err(e) = run() {
            error!(?e);
            process::exit(1);
        }
    });

    info!("Starting loop");

    std::thread::spawn(move || {
        let mut last = time::SystemTime::now();
        for () in recv {
            let now = time::SystemTime::now();
            if now.duration_since(last).unwrap_or_default().as_millis() < 500 {
                continue;
            }

            last = now;
            info!("Received update, rebuilding");
            if let Err(e) = build(root) {
                error!(?e);
            }
        }
    })
    .join()
    .map_err(|_| eyre!("build thread panicked"))
}

fn build(root: &Path) -> Result<()> {
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
