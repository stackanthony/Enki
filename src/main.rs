use std::{fs, io::Write, process::{Child, Stdio}};
use clap::{Parser, ValueEnum};
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
    command_args: Vec<String>
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
    // asserts path exists
    assert!(fs::exists(&args.image_path).unwrap());

    let entry_point = args.command_args.first().unwrap();
    println!("Entry point: {}", entry_point);

    let mut child = Command::new(entry_point).stdin(Stdio::piped()).spawn().expect("Failed to spawn child");
    container_setup(&mut child, &args.image_path, entry_point);

    // We need a child process who's chroot'd to the image path defined by args.image_path. chroot
    // then chdir.
    // we first need to setup the container namespaces and then 
}

fn container_setup(child: &mut Child, image_path: &str, entry_point: &str) {

    let byte_vec: &[u8] = &format!("sudo chroot {} {}", image_path, entry_point).into_bytes();
    println!("Byte vec: {}", std::str::from_utf8(byte_vec).unwrap());
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(byte_vec).expect("Failed to write to stdin of child");
    }

    let status = child.wait().expect("Failed to wait on child");
    println!("Child exited with: {}", status);
    //todo: shim sets up all of the namespaces, pivot root, etc for the container process.
}
