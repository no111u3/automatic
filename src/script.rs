use std::path::PathBuf;

use crate::promiscuous_list::PromiscousList;

#[derive(Debug, PartialEq)]
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
                "Config: {} doesn't exist",
                self.path.to_str().unwrap()
            ));
        }

        Ok(List::Promiscous(PromiscousList::new(vec![])))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn create() {
        let right_script = Script::new(PathBuf::from("."));

        assert_eq!(
            right_script.parse(),
            Ok(List::Promiscous(PromiscousList::new(vec![])))
        );

        let wrong_script = Script::new(PathBuf::new());

        assert_eq!(
            wrong_script.parse(),
            Err("Config:  doesn't exist".to_string())
        );
    }
}
