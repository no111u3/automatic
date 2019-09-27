use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::promiscuous_list::PromiscousList;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum List {
    Promiscous(PromiscousList),
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
            Ok(List::Promiscous(PromiscousList::new(vec![])))
        );
    }

    #[test]
    fn serde_script() {
        let item = List::Promiscous(PromiscousList::new(vec![]));

        let encoded = serde_yaml::to_string(&item).unwrap();

        assert_eq!(encoded, "---\nPromiscous:\n  items: []".to_string());

        let decoded: List = serde_yaml::from_str(&encoded).unwrap();

        assert_eq!(decoded, item);
    }
}
