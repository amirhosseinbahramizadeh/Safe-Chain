use std::collections::HashMap;

use secure_shield::prelude::*;

pub struct SecureVault {
    pub access: AccessControl,
    pub guard: ReentrancyGuard,
    pub pause: Pausable,
    pub timelock: TimelockManager,
    balances: HashMap<AccountId, u64>,
}

impl SecureVault {
    pub fn init(owner: AccountId) -> Self {
        Self {
            access: AccessControl::new(owner),
            guard: ReentrancyGuard::new(),
            pause: Pausable::new(),
            timelock: TimelockManager::new(86_400),
            balances: HashMap::new(),
        }
    }

    pub fn balance_of(&self, user: &str) -> u64 {
        *self.balances.get(user).unwrap_or(&0)
    }

    pub fn deposit(&mut self, caller: AccountId, amount: u64) -> SecureResult<()> {
        self.pause.when_not_paused()?;
        let _lock = self.guard.acquire()?;

        let current_balance = self.balance_of(&caller);
        let new_balance = SafeMath::add(current_balance, amount)?;
        self.balances.insert(caller, new_balance);

        Ok(())
    }

    pub fn withdraw(&mut self, caller: AccountId, amount: u64) -> SecureResult<()> {
        self.pause.when_not_paused()?;
        let _lock = self.guard.acquire()?;

        let current_balance = self.balance_of(&caller);

        if current_balance < amount {
            return Err(SecurityError::InsufficientBalance);
        }

        let new_balance = SafeMath::sub(current_balance, amount)?;
        self.balances.insert(caller, new_balance);

        Ok(())
    }

    pub fn emergency_stop(&mut self, caller: &str) -> SecureResult<()> {
        if !self.access.has_role(caller, "pauser") {
            return Err(SecurityError::Unauthorized);
        }

        self.pause.pause();
        Ok(())
    }

    pub fn resume_operations(&mut self, caller: &str) -> SecureResult<()> {
        if !self.access.has_role(caller, "pauser") {
            return Err(SecurityError::Unauthorized);
        }

        self.pause.unpause();
        Ok(())
    }

    pub fn propose_owner_transfer(
        &mut self,
        caller: &str,
        action_id: ActionId,
        current_time: u64,
        new_owner: AccountId,
    ) -> SecureResult<()> {
        if !self.access.has_role(caller, "governor") {
            return Err(SecurityError::Unauthorized);
        }

        let data = format!("transfer_owner:{}", new_owner);
        self.timelock.propose_action(action_id, current_time, data);
        Ok(())
    }

    pub fn execute_owner_transfer(
        &mut self,
        caller: &str,
        action_id: ActionId,
        current_time: u64,
    ) -> SecureResult<()> {
        if !self.access.has_role(caller, "governor") {
            return Err(SecurityError::Unauthorized);
        }

        let data = self.timelock.execute_action(action_id, current_time)?;
        let prefix = "transfer_owner:";

        if !data.starts_with(prefix) {
            return Err(SecurityError::InvalidActionData);
        }

        let new_owner = data[prefix.len()..].to_string();
        let current_owner = self.access.owner().to_string();

        self.access.transfer_ownership(&current_owner, new_owner)?;
        Ok(())
    }
}
