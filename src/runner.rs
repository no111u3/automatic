use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output};

struct Runner {
    cmd: Command,
}

impl Runner {
    pub fn new<S: AsRef<OsStr>>(cmd: S, args: Vec<S>) -> Runner {
        let mut cmd = Command::new(cmd);
        for arg in args {
            cmd.arg(arg);
        }

        Runner { cmd }
    }

    pub fn run(&mut self) -> io::Result<Output> {
        self.cmd.output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let result = Runner::new("true", vec![])
            .run()
            .expect("failed to execute process");
        assert!(result.status.success());

        let result = Runner::new("true", vec!["1", "2", "3"])
            .run()
            .expect("failed to execute process");
        assert!(result.status.success());

        let result = Runner::new("false", vec![])
            .run()
            .expect("failed to execute process");
        assert!(!result.status.success());

        let result = Runner::new("false", vec!["1", "2", "3"])
            .run()
            .expect("failed to execute process");
        assert!(!result.status.success());
    }
}
