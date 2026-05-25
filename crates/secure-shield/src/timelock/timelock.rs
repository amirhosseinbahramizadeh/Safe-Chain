use std::collections::HashMap;

use crate::errors::SecurityError;
use crate::result::SecureResult;

pub type ActionId = u128;

#[derive(Debug, Clone)]
pub struct TimelockAction {
    pub execution_time: u64,
    pub data: String,
    pub is_executed: bool,
}

#[derive(Debug)]
pub struct TimelockManager {
    delay: u64,
    proposals: HashMap<ActionId, TimelockAction>,
}

impl TimelockManager {
    pub fn new(delay: u64) -> Self {
        Self {
            delay,
            proposals: HashMap::new(),
        }
    }

    pub fn delay(&self) -> u64 {
        self.delay
    }

    pub fn propose_action(&mut self, id: ActionId, current_time: u64, data: String) {
        let execution_time = current_time.saturating_add(self.delay);

        self.proposals.insert(
            id,
            TimelockAction {
                execution_time,
                data,
                is_executed: false,
            },
        );
    }

    pub fn execute_action(&mut self, id: ActionId, current_time: u64) -> SecureResult<String> {
        let action = self
            .proposals
            .get_mut(&id)
            .ok_or(SecurityError::ActionNotProposed)?;

        if action.is_executed {
            return Err(SecurityError::ActionAlreadyExecuted);
        }

        if current_time < action.execution_time {
            return Err(SecurityError::TimelockNotMet);
        }

        action.is_executed = true;
        Ok(action.data.clone())
    }

    pub fn get_action(&self, id: ActionId) -> Option<&TimelockAction> {
        self.proposals.get(&id)
    }
}
