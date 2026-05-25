use crate::errors::SecurityError;
use crate::result::SecureResult;

#[derive(Debug, Default)]
pub struct Pausable {
    is_paused: bool,
}

impl Pausable {
    pub fn new() -> Self {
        Self { is_paused: false }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn when_not_paused(&self) -> SecureResult<()> {
        if self.is_paused {
            Err(SecurityError::ContractPaused)
        } else {
            Ok(())
        }
    }
}
