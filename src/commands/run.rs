use nix::mount::{MsFlags, mount};
use nix::sched::{CloneFlags, unshare};
use nix::unistd::{chdir, chroot, sethostname};
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub fn execute_run(image_path: &str, entry_point: &str) -> Result<ExitStatus, std::io::Error> {
    let image_path = PathBuf::from(image_path);
    // asserts path exists
    assert!(fs::exists(&image_path).unwrap());

    println!("Entry point: {}", entry_point);

    unshare(CloneFlags::CLONE_NEWPID).expect("Failed to create pid namespace");

    let mut cmd = Command::new("/bin/bash");

    unsafe {
        cmd.pre_exec(move || {
            let flags =
                CloneFlags::CLONE_NEWNET | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS;

            unshare(flags)?;
            mount(
                None::<&str>,
                Path::new("/"),
                None::<&str>,
                MsFlags::MS_REC | MsFlags::MS_PRIVATE,
                None::<&str>,
            )?;
            chroot(image_path.as_path())?;
            chdir("/")?;

            sethostname("enki-container")?;
            mount(
                Some("proc"),
                Path::new("/proc"),
                Some("proc"),
                MsFlags::empty(),
                None::<&str>,
            )?;

            Ok(())
        });
    }

    let mut child = cmd.spawn()?;
    let status = child.wait()?;
    println!("Child exited with status: {}", status);

    Ok(status)
}
