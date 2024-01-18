use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use std::process::Command;

pub fn run_process(cmd: &mut Command) -> Result<String> {
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
