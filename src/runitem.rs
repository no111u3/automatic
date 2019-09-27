use serde::{Deserialize, Serialize};

use crate::runner::{io, Output, Runned, Runner};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunItem {
    name: String,
    args: Vec<String>,
}

impl RunItem {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    pub fn run(&self) -> io::Result<Output> {
        Runner::new(self.name.clone(), self.args.clone()).run()
    }

    pub fn run_async(&self) -> Runned {
        Runner::new(self.name.clone(), self.args.clone()).run_async()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let result = RunItem::new("true".to_string(), vec![])
            .run()
            .expect("failed to execute process");
        assert!(result.status.success());

        let result = RunItem::new(
            "true".to_string(),
            vec!["1", "2", "3"]
                .iter()
                .cloned()
                .map(String::from)
                .collect(),
        )
        .run()
        .expect("failed to execute process");
        assert!(result.status.success());
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
