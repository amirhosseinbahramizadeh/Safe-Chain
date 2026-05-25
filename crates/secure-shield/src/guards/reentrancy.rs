use crate::errors::SecurityError;
use crate::result::SecureResult;

#[derive(Debug, Default)]
pub struct ReentrancyGuard {
    locked: bool,
}

pub struct ReentrancyLock<'a> {
    pub(crate) guard: &'a mut ReentrancyGuard,
}

impl<'a> Drop for ReentrancyLock<'a> {
    fn drop(&mut self) {
        self.guard.locked = false;
    }
}

impl ReentrancyGuard {
    pub fn new() -> Self {
        Self { locked: false }
    }

    pub fn acquire(&mut self) -> SecureResult<ReentrancyLock<'_>> {
        if self.locked {
            return Err(SecurityError::ReentrancyDetected);
        }

        self.locked = true;
        Ok(ReentrancyLock { guard: self })
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
}
