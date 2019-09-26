use std::ffi::OsStr;
pub use std::io;
pub use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Output, Stdio,
};

pub struct Runned {
    process: Child,
}

impl Runned {
    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        match self.process.try_wait() {
            Ok(Some(status)) => Ok(status),
            Ok(None) => self.process.wait(),
            Err(e) => Err(e),
        }
    }

    pub fn get_stdin(&mut self) -> Option<ChildStdin> {
        self.process.stdin.take()
    }

    pub fn get_stdout(&mut self) -> Option<ChildStdout> {
        self.process.stdout.take()
    }

    pub fn get_stderr(&mut self) -> Option<ChildStderr> {
        self.process.stderr.take()
    }
}

pub struct Runner {
    cmd: Command,
}

impl Runner {
    pub fn new<S: AsRef<OsStr>>(cmd: S, args: Vec<S>) -> Self {
        let mut cmd = Command::new(cmd);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.args(args);

        Runner { cmd }
    }

    pub fn set_stdin<T: Into<Stdio>>(&mut self, stdin: T) -> &mut Self {
        self.cmd.stdin(stdin);
        self
    }

    pub fn set_stdout<T: Into<Stdio>>(&mut self, stdout: T) -> &mut Self {
        self.cmd.stdout(stdout);
        self
    }

    pub fn set_stderr<T: Into<Stdio>>(&mut self, stderr: T) -> &mut Self {
        self.cmd.stderr(stderr);
        self
    }

    pub fn run(&mut self) -> io::Result<Output> {
        self.cmd.output()
    }

    pub fn run_async(&mut self) -> Runned {
        Runned {
            process: self.cmd.spawn().expect("failed to execute process"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

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

    #[test]
    fn create_async() {
        let result = Runner::new("true", vec![])
            .run_async()
            .wait()
            .expect("failed to wait");
        assert!(result.success());

        let result = Runner::new("true", vec!["1", "2", "3"])
            .run_async()
            .wait()
            .expect("failed to wait");
        assert!(result.success());

        let result = Runner::new("false", vec![])
            .run_async()
            .wait()
            .expect("failed to wait");
        assert!(!result.success());

        let result = Runner::new("false", vec!["1", "2", "3"])
            .run_async()
            .wait()
            .expect("failed to wait");
        assert!(!result.success());
    }

    #[test]
    fn std_in_out() {
        let mut r_async = Runner::new("cat", vec![]).run_async();
        {
            let mut input = r_async.get_stdin().unwrap();
            write!(input, "123456").expect("failed in write to pipe");
        }
        let mut output = r_async.get_stdout().unwrap();
        let mut line = String::new();
        output
            .read_to_string(&mut line)
            .expect("failed in read to string");
        assert_eq!(line, "123456");

        let mut error = r_async.get_stderr().unwrap();
        let mut line = String::new();
        error
            .read_to_string(&mut line)
            .expect("failed in read to string");
        assert_eq!(line, "");
    }

    #[test]
    fn std_pipeline() {
        let mut r_async = Runner::new("cat", vec![]).run_async();
        {
            let mut input = r_async.get_stdin().unwrap();
            write!(input, "123456").expect("failed in write to pipe");
        }

        let mut r_async_two = Runner::new("cat", vec![])
            .set_stdin(r_async.get_stdout().unwrap())
            .run_async();

        let mut output = r_async_two.get_stdout().unwrap();

        let mut line = String::new();
        output
            .read_to_string(&mut line)
            .expect("failed in read to string");
        assert_eq!(line, "123456");

        let mut error = r_async.get_stderr().unwrap();
        let mut line = String::new();
        error
            .read_to_string(&mut line)
            .expect("failed in read to string");
        assert_eq!(line, "");
    }
}
