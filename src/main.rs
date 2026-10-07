use clap::{Parser, ValueEnum};
use std::io;
use std::process::{ChildStdin, Command};
use std::{
    fs,
    io::Write,
    process::{Stdio},
    thread
};
use nix::unistd::sethostname;

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
    // asserts path exists
    assert!(fs::exists(&args.image_path).unwrap());

    let entry_point = args.command_args.first().unwrap();
    println!("Entry point: {}", entry_point);

    //TODO: Rewrite raw bash commands into utilizing the nix crate
    let initial_commands = format!("sudo unshare -p -f --mount-proc=./rootfs/proc \
        sudo chroot {} {};", args.image_path, entry_point);
    let child = Command::new("bash")
        .args(["-c", &initial_commands])
        .spawn()
        .expect("Failed to start shell");

    println!("Child pid: {}", child.id());
    // let stdin = child.stdin.take().expect("Failed to open stdin");
    child.wait_with_output().expect("Failed to read output");
    //
    // // container_setup(stdin, &args.image_path, entry_point);
    // let mut child = Command::new(entry_point).stdin(Stdio::piped()).spawn().expect("Failed to spawn child");

    // We need a child process who's chroot'd to the image path defined by args.image_path. chroot
    // then chdir.
    // we first need to setup the container namespaces and then
}
// fn container_setup(mut stdin: ChildStdin, image_path : &str, entry_point: &str) {
//     let _ = writeln!(stdin, "sudo chroot {} {}", image_path, entry_point);
//     let _ = writeln!(stdin, "sudo chdir /");
//     stdin.flush();
//
//     thread::spawn(move || {
//         let mut parent_stdin = io::stdin();
//         if let Err(e) = io::copy(&mut parent_stdin, &mut stdin) {
//             eprintln!("Error forwarding stdin: {}", e);
//         }
//     });
//
//     // let mut child = Command::new("bash")
//     //     .stdin(Stdio::inherit())
//     //     .stdout(Stdio::inherit())
//     //     .stderr(Stdio::inherit())
//     //     .spawn()
//     // .expect("Failed to start shell");
//     //
//     // let status = child.wait().expect("Failed to wait on child");
// }
