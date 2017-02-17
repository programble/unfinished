extern crate nix;

use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::RawFd;

use nix::libc::{STDIN_FILENO, STDOUT_FILENO, pid_t};
use nix::sys::wait::{self, WaitStatus};
use nix::unistd::{self, ForkResult};

#[derive(Debug, Clone)]
struct Job<'a> {
    procs: Vec<Process<'a>>,
}

impl<'a> From<&'a str> for Job<'a> {
    fn from(input: &'a str) -> Self {
        if input.trim().is_empty() {
            return Job { procs: vec![] };
        }

        let mut job = Job {
            procs: input.split('|').map(Process::from).collect(),
        };

        for i in 0..(job.procs.len() - 1) {
            let (read, write) = unistd::pipe().unwrap();
            job.procs[i].fds.push((STDOUT_FILENO, write));
            job.procs[i + 1].fds.push((STDIN_FILENO, read));
        }

        job
    }
}

impl<'a> Job<'a> {
    fn run(&mut self) -> i8 {
        for process in &mut self.procs {
            process.start();
        }
        let mut exit_code = 0;
        for process in &self.procs {
            exit_code = process.wait();
        }
        exit_code
    }
}

#[derive(Debug, Clone)]
struct Process<'a> {
    args: Vec<&'a str>,
    fds: Vec<(RawFd, RawFd)>,
    pid: Option<pid_t>,
    builtin_exit_code: Option<i8>,
}

impl<'a> From<&'a str> for Process<'a> {
    fn from(input: &'a str) -> Self {
        Process {
            args: input.split_whitespace().collect(),
            fds: vec![],
            pid: None,
            builtin_exit_code: None,
        }
    }
}

impl<'a> Process<'a> {
    fn start(&mut self) {
        match self.args[0] {
            "cd" => self.builtin_exit_code = Some(builtin_cd(&self.args[1..])),
            _ => self.execute(),
        }
    }

    fn execute(&mut self) {
        match unistd::fork().unwrap() {
            ForkResult::Parent { child } => {
                self.pid = Some(child);
            },

            ForkResult::Child => {
                for &(new, old) in &self.fds {
                    unistd::dup2(old, new).unwrap();
                }

                let c_cmd = CString::new(self.args[0]).unwrap();
                let c_args: Vec<CString> = self.args.iter()
                    .map(|&s| CString::new(s).unwrap())
                    .collect();

                unistd::execvp(&c_cmd, &c_args).unwrap();
                unreachable!()
            },
        }
    }

    fn wait(&self) -> i8 {
        if let Some(exit_code) = self.builtin_exit_code {
            return exit_code;
        }
        match wait::waitpid(self.pid.unwrap(), None).unwrap() {
            WaitStatus::Exited(_, code) => code,
            s => panic!("my child got {:?}", s),
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

        let mut job = Job::from(input.as_str());

        println!("{:?}", job);

        exit_code = job.run();
    }

    println!("");
}
