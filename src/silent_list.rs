//! Silent run items, no output but aborts if one fail start or fail exit
//!
//! # Example
//!```
//!use automatic::silent_list::SilentList;
//!use automatic::runitem::RunItem;
//!use automatic::run::Run;
//!let items = vec![
//!    RunItem::new("true".to_string(), vec![]),
//!    RunItem::new("true".to_string(), vec![]),
//!    RunItem::new("true".to_string(), vec![]),
//!];
//!
//!let result = SilentList::new(items)
//!    .run()
//!    .status()
//!    .expect("failed to execute process");
//!assert!(result.success());
//!```

use serde::{Deserialize, Serialize};

use crate::run::{ExitStatus, Run, RunStatus};
use crate::runitem::RunItem;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SilentList {
    items: Vec<RunItem>,
}

impl SilentList {
    pub fn new(items: Vec<RunItem>) -> Self {
        Self { items }
    }

    fn run_internal(&self) -> Result<(), String> {
        for item in self.items.iter() {
            let result = match item.run().status() {
                Err(e) => {
                    return Err(e.to_string());
                }
                Ok(result) => result,
            };

            if !result.success() {
                return Err(format!(
                    "runned item return fail execution state with code: {}",
                    result.code().unwrap()
                ));
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

impl Run for SilentList {
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

        let result = SilentList::new(items)
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

        let runner = SilentList::new(items);
        assert_eq!(
            runner.run().status(),
            Err("No such file or directory (os error 2)".to_string())
        );

        let items = vec![
            RunItem::new("true".to_string(), vec![]),
            RunItem::new("false".to_string(), vec![]),
            RunItem::new("true".to_string(), vec![]),
        ];

        let runner = SilentList::new(items);
        assert_eq!(
            runner.run().status(),
            Err("runned item return fail execution state with code: 1".to_string())
        );
    }
}
