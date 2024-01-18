use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use fs_extra::dir::CopyOptions;
use std::{path::Path, process::Command};

pub fn run_process(cmd: &mut Command) -> Result<String> {
    fn run_process_inner(cmd: &mut Command) -> Result<String> {
        let name = cmd.get_program().to_os_string();
        let output = cmd
            .output()
            .wrap_err(format!("failed to spawn process {name:?}"))?;

        if !output.status.success() {
            bail!(
                "command returned error: {}",
                String::from_utf8(output.stderr).wrap_err("stderr is not UTF-8")?
            );
        }

        Ok(String::from_utf8(output.stdout).wrap_err("stdout is not UTF-8")?)
    }
    run_process_inner(cmd).wrap_err(format!(
        "{} {}",
        cmd.get_program().to_str().unwrap(),
        cmd.get_args()
            .map(|arg| format!("'{}'", arg.to_str().unwrap()))
            .collect::<Vec<_>>()
            .join(" ")
    ))
}

pub fn create_dir_if_not_exist(p: &Path) -> Result<()> {
    match std::fs::create_dir(p) {
        Ok(()) => debug!(?p, "Created directory"),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        e => return e.wrap_err("failed to create submodules"),
    }
    Ok(())
}

pub fn cp_r(from: &Path, to: &Path) -> Result<()> {
    fs_extra::copy_items(
        &[from],
        to,
        &CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..CopyOptions::default()
        },
    )
    .wrap_err(format!("copying to {}", to.display()))?;
    Ok(())
}

pub fn cp_content(from: &Path, to: &Path) -> Result<()> {
    fs_extra::dir::copy(
        from,
        to,
        &CopyOptions {
            overwrite: true,
            copy_inside: true,
            content_only: true,
            ..CopyOptions::default()
        },
    )
    .wrap_err(format!("copying to {}", to.display()))?;
    Ok(())
}
