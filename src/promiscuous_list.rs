use serde::{Deserialize, Serialize};

use crate::runitem::RunItem;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PromiscousList {
    items: Vec<RunItem>,
}

impl PromiscousList {
    pub fn new(items: Vec<RunItem>) -> Self {
        Self { items }
    }

    pub fn run(&self) -> Result<(), String> {
        for item in self.items.iter() {
            if let Err(e) = item.run() {
                return Err(format!("{}", e));
            }
        }
        Ok(())
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

        let runner = PromiscousList::new(items);
        assert_eq!(runner.run(), Ok(()));
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
            runner.run(),
            Err("No such file or directory (os error 2)".to_string())
        );
    }
}
