//! Collect run parameters and run command
//!
//! # Example
//!```
//!use automatic::runitem::RunItem;
//!use automatic::run::Run;
//!
//!let result = RunItem::new("true".to_string(), vec![])
//!    .run()
//!    .status()
//!    .expect("failed to execute process");
//!assert!(result.success());
//!```

use serde::{Deserialize, Serialize};

use crate::run::{ExitStatus, Run, RunMap, RunStatus};
use crate::runner::{self, io, Output, Runned, Runner};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunItem {
    name: String,
    args: Vec<String>,
}

impl RunItem {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    pub fn run_async(&self) -> Runned {
        Runner::new(self.name.clone(), self.args.clone()).run_async()
    }
}

struct RunItemStatus {
    status: io::Result<Output>,
}

struct StatusHelper {
    status: runner::ExitStatus,
}

impl ExitStatus for StatusHelper {
    fn success(&self) -> bool {
        self.status.success()
    }

    fn code(&self) -> Option<i32> {
        self.status.code()
    }
}

impl RunStatus for RunItemStatus {
    fn status(&self) -> Result<Box<dyn ExitStatus>, String> {
        match &self.status {
            Ok(ok) => Ok(Box::new(StatusHelper { status: ok.status })),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl Run for RunItem {
    fn run(&self) -> Box<dyn RunStatus> {
        Box::new(RunItemStatus {
            status: Runner::new(self.name.clone(), self.args.clone()).run(),
        })
    }
}

impl RunMap<Runner> for RunItem {
    fn run_map<F: FnOnce(&mut Runner) -> &mut Runner>(&self, op: F) -> Box<dyn RunStatus> {
        Box::new(RunItemStatus {
            status: op(&mut Runner::new(self.name.clone(), self.args.clone())).run(),
        })
    }
}

#[cfg(test)]
mod tests {
    use os_pipe::pipe;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn create() {
        let result = RunItem::new("true".to_string(), vec![])
            .run()
            .status()
            .expect("failed to execute process");
        assert!(result.success());

        let result = RunItem::new(
            "true".to_string(),
            vec!["1", "2", "3"]
                .iter()
                .cloned()
                .map(String::from)
                .collect(),
        )
        .run()
        .status()
        .expect("failed to execute process");
        assert!(result.success());
    }

    #[test]
    fn create_with_output_to_pipe() {
        let (mut reader, writer) = pipe().unwrap();

        let result = RunItem::new("ls".to_string(), vec![])
            .run_map(|r| r.set_stdout(writer))
            .status()
            .expect("failed to execute process");
        assert!(result.success());

        let mut output = String::new();
        reader.read_to_string(&mut output).unwrap();
        assert_eq!(
            output,
            "Cargo.lock\nCargo.toml\nLICENSE\nREADME.md\nsrc\ntarget\ntests\n"
        );
    }

    #[test]
    fn create_async() {
        let result = RunItem::new("true".to_string(), vec![])
            .run_async()
            .wait()
            .expect("failed to wait");
        assert!(result.success());

        let result = RunItem::new(
            "true".to_string(),
            vec!["1", "2", "3"]
                .iter()
                .cloned()
                .map(String::from)
                .collect(),
        )
        .run_async()
        .wait()
        .expect("failed to wait");
        assert!(result.success());
    }
}
