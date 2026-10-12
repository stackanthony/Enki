use clap::{Parser, ValueEnum};
use nix::mount::{MsFlags, mount};
use nix::sched::{CloneFlags, unshare};
use nix::unistd::{chdir, chroot, sethostname};
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(ValueEnum, Clone, Copy, Debug)]
enum EnkiCommand {
    Run,
}

#[derive(Parser, Debug)]
struct Cli {
    command: EnkiCommand,
    image_path: String,

    #[arg(num_args = 1..)]
    command_args: Vec<String>,
}

//TODO: enki <command> <image_path> <command_args n+>
fn main() {
    let args = Cli::parse();

    println!("args: {:?}", args);
    //TODO: Eventually probably going to be other commands here
    match args.command {
        EnkiCommand::Run => run(&args),
    }
}

fn run(args: &Cli) {
    let image_path = PathBuf::from(&args.image_path);
    // asserts path exists
    assert!(fs::exists(&image_path).unwrap());

    let entry_point = args.command_args.first().unwrap();
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

    match cmd.spawn() {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait on child");
            println!("Child exited with status: {}", status);
        }
        Err(e) => println!("Failed to spawn child: {:?}", e),
    }
}
