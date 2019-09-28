//! Promiscous run items, no output but aborts if one fail

use serde::{Deserialize, Serialize};

use crate::run::{ExitStatus, Run, RunStatus};
use crate::runitem::RunItem;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PromiscousList {
    items: Vec<RunItem>,
}

impl PromiscousList {
    pub fn new(items: Vec<RunItem>) -> Self {
        Self { items }
    }

    fn run_internal(&self) -> Result<(), String> {
        for item in self.items.iter() {
            if let Err(e) = item.run().status() {
                return Err(e.to_string());
            }
        }
        Ok(())
    }
}

struct StatusHelper {}

impl ExitStatus for StatusHelper {}

struct RunItemStatus {
    status: Result<(), String>,
}

impl RunStatus for RunItemStatus {
    fn status(&self) -> Result<Box<dyn ExitStatus>, String> {
        match &self.status {
            Ok(_) => Ok(Box::new(StatusHelper {})),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Run for PromiscousList {
    fn run(&self) -> Box<dyn RunStatus> {
        Box::new(RunItemStatus {
            status: self.run_internal(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::runitem::RunItem;

    use super::*;

    #[test]
    fn success_run() {
        let items = vec![
            RunItem::new("true".to_string(), vec![]),
            RunItem::new("true".to_string(), vec![]),
            RunItem::new("true".to_string(), vec![]),
        ];

        let result = PromiscousList::new(items)
            .run()
            .status()
            .expect("failed to execute process");
        assert!(result.success());
    }

    #[test]
    fn fail_run() {
        let items = vec![
            RunItem::new("true".to_string(), vec![]),
            RunItem::new("bla bla".to_string(), vec![]),
            RunItem::new("true".to_string(), vec![]),
        ];

        let runner = PromiscousList::new(items);
        assert_eq!(
            runner.run().status(),
            Err("No such file or directory (os error 2)".to_string())
        );
    }
}
