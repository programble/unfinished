extern crate nix;

use std::env;
use std::ffi::CString;
use std::io::{self, Write};

use nix::sys::wait::{self, WaitStatus};
use nix::unistd::{self, ForkResult};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut exit_code = 0;

    loop {
        let cwd = unistd::getcwd().unwrap();
        if exit_code == 0 {
            write!(stdout, "{} $ ", cwd.display()).unwrap();
        } else {
            write!(stdout, "{} ({}) $ ", cwd.display(), exit_code).unwrap();
        }
        stdout.flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        if input.is_empty() { break }

        let mut args = input.split_whitespace();
        let cmd = match args.next() {
            Some(c) => c,
            None => continue,
        };

        exit_code = match cmd {
            "cd" => builtin_cd(args),
            _ => execute(cmd, args),
        };
    }

    println!("");
}

fn builtin_cd<'a, I>(mut args: I) -> i8
where I: Iterator<Item = &'a str> {
    let path = match args.next() {
        Some(p) => p.into(),
        None => env::home_dir().unwrap(),
    };

    match unistd::chdir(&path) {
        Ok(_) => 0,
        Err(e) => {
            println!("cd: {}", e);
            1
        },
    }
}

fn execute<'a, I>(cmd: &str, args: I) -> i8
where I: Iterator<Item = &'a str> {
    match unistd::fork().unwrap() {
        ForkResult::Parent { child } => {
            match wait::waitpid(child, None).unwrap() {
                WaitStatus::Exited(_, code) => code,
                s => panic!("my child got {:?}", s),
            }
        },

        ForkResult::Child => {
            let c_cmd = CString::new(cmd).unwrap();
            let mut c_args = Vec::new();
            c_args.push(c_cmd.clone());
            c_args.extend(args.map(|a| CString::new(a).unwrap()));

            unistd::execvp(&c_cmd, &c_args).unwrap();
            unreachable!()
        },
    }
}
