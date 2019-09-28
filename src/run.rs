//! Run trait for unificate run any items

use std::fmt;

/// Exit status collect trait
///
/// Status collect item run state but have default state
/// `true` for `success()` method and `None` for `code()`
pub trait ExitStatus {
    /// Status of runned item - `true` for 0 return code or for not implemented
    /// state
    fn success(&self) -> bool {
        true
    }

    /// Exit run code - `Some<i32>` for any run state, `None` for failed to run
    /// or not implemented state
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

/// Run status collect trait
///
/// Status collect current item state and other run item attributes
pub trait RunStatus {
    /// Current Exit status of runned program or error of run command
    fn status(&self) -> Result<Box<dyn ExitStatus>, String>;
}

/// Run item trait
///
/// Interface for synchoniosly run item
pub trait Run {
    /// Synchoniosly run item, return it's run status
    fn run(&self) -> Box<dyn RunStatus>;
}
