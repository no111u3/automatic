pub trait ExitStatus {
    fn success(&self) -> bool;

    fn code(&self) -> Option<i32> {
        None
    }
}

pub trait RunStatus {
    fn status(&self) -> Result<Box<dyn ExitStatus>, String>;
}

pub trait Run {
    fn run(&self) -> Box<dyn RunStatus>;
}
