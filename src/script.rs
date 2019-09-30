//! Yaml script parser and run items list collector
//!
//! # Example
//!```
//!use std::path::PathBuf;
//!
//!use automatic::script::Script;
//!use automatic::run::Run;
//!
//!let items = vec![
//!    "tests/test_script_for_run.yaml",
//!    "tests/test_script_for_run_silent.yaml",
//!];
//!
//!for item in items {
//!    let script = Script::new(PathBuf::from(item));
//!
//!    let runner = script.parse().unwrap();
//!
//!    let result = runner.run().status().expect("failed to execute process");
//!    assert!(result.success());
//!}
//!```


use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::promiscuous_list::PromiscuousList;
use crate::silent_list::SilentList;

use crate::run::{Run, RunStatus};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum List {
    Promiscuous(PromiscuousList),
    Silent(SilentList),
}

impl Run for List {
    fn run(&self) -> Box<dyn RunStatus> {
        match self {
            List::Promiscuous(list) => list.run(),
            List::Silent(list) => list.run(),
        }
    }
}

pub struct Script {
    path: PathBuf,
}

impl Script {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn parse(&self) -> Result<List, String> {
        if !self.path.exists() {
            return Err(format!(
                "Script: {} doesn't exist",
                self.path.to_str().unwrap()
            ));
        }

        let mut f = File::open(self.path.clone())
            .or_else(|e| Err(format!("fail to open script with error: {}", e)))?;

        let mut s = String::new();
        f.read_to_string(&mut s)
            .or_else(|e| Err(format!("fail to read script with error: {}", e)))?;

        match serde_yaml::from_str(&s) {
            Ok(entity) => Ok(entity),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::runitem::RunItem;

    use super::*;

    #[test]
    fn create() {
        let wrong_script = Script::new(PathBuf::from("."));

        assert_eq!(
            wrong_script.parse(),
            Err("fail to read script with error: Is a directory (os error 21)".to_string())
        );

        let wrong_script = Script::new(PathBuf::new());

        assert_eq!(
            wrong_script.parse(),
            Err("Script:  doesn't exist".to_string())
        );

        let right_script = Script::new(PathBuf::from("tests/test_script.yaml"));

        assert_eq!(
            right_script.parse(),
            Ok(List::Promiscuous(PromiscuousList::new(vec![])))
        );
    }

    #[test]
    fn serde_script() {
        let item = List::Promiscuous(PromiscuousList::new(vec![]));

        let encoded = serde_yaml::to_string(&item).unwrap();

        assert_eq!(encoded, "---\nPromiscuous:\n  items: []".to_string());

        let decoded: List = serde_yaml::from_str(&encoded).unwrap();

        assert_eq!(decoded, item);

        let items = List::Promiscuous(PromiscuousList::new(vec![
            RunItem::new("true".to_string(), vec![]),
            RunItem::new("true".to_string(), vec![]),
            RunItem::new("true".to_string(), vec![]),
        ]));

        let encoded = serde_yaml::to_string(&items).unwrap();

        assert_eq!(
            encoded,
            "---\nPromiscuous:\
             \n  items:\
             \n    - name: \"true\"\
             \n      args: []\
             \n    - name: \"true\"\
             \n      args: []\
             \n    - name: \"true\"\
             \n      args: []"
                .to_string()
        );

        let decoded: List = serde_yaml::from_str(&encoded).unwrap();

        assert_eq!(decoded, items);
    }

    #[test]
    fn test_run_script() {
        let items = vec![
            "tests/test_script_for_run.yaml",
            "tests/test_script_for_run_silent.yaml",
        ];

        for item in items {
            let script = Script::new(PathBuf::from(item));

            let runner = script.parse().unwrap();

            let result = runner.run().status().expect("failed to execute process");
            assert!(result.success());
        }
    }
}
