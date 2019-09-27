use serde::{Deserialize, Serialize};

use crate::run::{Run, RunStatus};
use crate::runner::{io, ExitStatus, Output, Runned, Runner};

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

pub struct RunItemStatus {
    status: io::Result<Output>,
}

impl RunStatus for RunItemStatus {
    fn status(&self) -> Result<ExitStatus, String> {
        match &self.status {
            Ok(ok) => Ok(ok.status),
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

#[cfg(test)]
mod tests {
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
