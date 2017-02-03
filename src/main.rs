extern crate nix;

use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::RawFd;
use std::path::Path;

use nix::sys::wait::{self, WaitStatus};
use nix::unistd::{self, ForkResult};

#[derive(Debug, Clone, Copy)]
enum Redirect<'a> {
    Fd(RawFd),
    File(&'a Path),
}

#[derive(Debug)]
struct Command<'a> {
    args: Vec<&'a str>,
    fds: Vec<(RawFd, Redirect<'a>)>,
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(input: &'a str) -> Self {
        Command {
            args: input.split_whitespace().collect(),
            fds: vec![],
        }
    }
}

impl<'a> Command<'a> {
    fn run(&self) -> i8 {
        match self.args[0] {
            "cd" => builtin_cd(&self.args[1..]),
            _ => self.execute(),
        }
    }

    fn execute(&self) -> i8 {
        match unistd::fork().unwrap() {
            ForkResult::Parent { child } => {
                match wait::waitpid(child, None).unwrap() {
                    WaitStatus::Exited(_, code) => code,
                    s => panic!("my child got {:?}", s),
                }
            },

            ForkResult::Child => {
                let c_cmd = CString::new(self.args[0]).unwrap();
                let c_args: Vec<CString> = self.args.iter()
                    .map(|&s| CString::new(s).unwrap())
                    .collect();

                for &(old, new) in &self.fds {
                    match new {
                        Redirect::Fd(newfd) => { unistd::dup2(old, newfd).unwrap(); },
                        _ => unimplemented!(),
                    }
                }

                unistd::execvp(&c_cmd, &c_args).unwrap();
                unreachable!();
            },
        }
    }
}

fn builtin_cd(args: &[&str]) -> i8 {
    let path = match args.first() {
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

        let mut cmds: Vec<Command> = input.split('|').map(Command::from).collect();

        for pair in cmds.chunks_mut(2) {
            let (read, write) = unistd::pipe().unwrap();
            pair[0].fds.push((1, Redirect::Fd(write)));
            pair[1].fds.push((0, Redirect::Fd(read)));
        }

        println!("{:?}", cmds);

        for cmd in &cmds {
            exit_code = cmd.run();
        }
    }

    println!("");
}
