use std::fmt;

pub trait ExitStatus {
    fn success(&self) -> bool {
        true
    }

    fn code(&self) -> Option<i32> {
        None
    }
}

impl fmt::Debug for dyn ExitStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "success: {}, code: {}",
            self.success(),
            self.code().unwrap_or(0)
        )
    }
}

impl PartialEq for dyn ExitStatus {
    fn eq(&self, other: &Self) -> bool {
        self.success() == other.success() && self.code() == other.code()
    }
}

pub trait RunStatus {
    fn status(&self) -> Result<Box<dyn ExitStatus>, String>;
}

pub trait Run {
    fn run(&self) -> Box<dyn RunStatus>;
}
