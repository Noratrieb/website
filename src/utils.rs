use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use std::{fs, io, os::unix::ffi::OsStrExt, path::{Path, PathBuf}, process::Command};

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

        String::from_utf8(output.stdout).wrap_err("stdout is not UTF-8")
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
    copy_fn(from, to, |content, _, _| Ok(content))
}

pub struct CopyOpts {
    pub dest_path: PathBuf,
}

pub fn copy_fn(
    from: &Path,
    to: &Path,
    mut map: impl FnMut(Vec<u8>, Option<&str>, &mut CopyOpts) -> Result<Vec<u8>>,
) -> Result<()> {
    let mut worklist = vec![from.to_owned()];

    while let Some(ref item) = worklist.pop() {
        let mut process = || -> Result<()> {
            let meta = fs::metadata(item).wrap_err("getting metadata")?;
            let relative = item.strip_prefix(from).wrap_err("subpath stripping")?;
            let dest = to.join(relative);

            if meta.is_dir() {
                let items = fs::read_dir(item).wrap_err("read_dir")?;
                for item in items {
                    let item = item.wrap_err("entry")?;
                    worklist.push(item.path());
                }

                match fs::create_dir_all(&dest) {
                    Ok(()) => {}
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                    e => return e.wrap_err_with(|| format!("creating {}", dest.display())),
                }
            } else {
                let content = fs::read(item).wrap_err("reading file")?;
                let ext = match item.extension() {
                    Some(ext) => Some(
                        std::str::from_utf8(ext.as_bytes())
                            .wrap_err("file extension is invalid UTF-8")?,
                    ),
                    None => None,
                };
                let mut opts = CopyOpts {
                    dest_path: dest.clone(),
                };
                let result = map(content, ext, &mut opts).wrap_err("applying mapping")?;
                fs::write(opts.dest_path, result)
                    .wrap_err_with(|| format!("creating {}", dest.display()))?;
            }
            Ok(())
        };
        process().wrap_err_with(|| format!("copying {}", item.display()))?;
    }

    Ok(())
}
