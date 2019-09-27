pub use std::process::ExitStatus;

pub trait RunStatus {
    fn status(&self) -> Result<ExitStatus, String>;
}

pub trait Run {
    fn run(&self) -> Box<dyn RunStatus>;
}
