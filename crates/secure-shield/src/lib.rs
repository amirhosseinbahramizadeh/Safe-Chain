pub mod secure_shield {
    use std::collections::{HashMap, HashSet};

    pub type AccountId = String;
    pub type ActionId = u128;

    // =========================================================
    // Errors
    // =========================================================
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum SecurityError {
        Unauthorized,
        ContractPaused,
        ReentrancyDetected,
        ArithmeticOverflow,
        TimelockNotMet,
        ActionNotProposed,
        ActionAlreadyExecuted,
        RoleNotFound,
        InsufficientBalance,
        InvalidActionData,
    }

    pub type SecureResult<T> = Result<T, SecurityError>;

    // =========================================================
    // 1. Safe Math
    // =========================================================
    pub struct SafeMath;

    impl SafeMath {
        pub fn add(a: u64, b: u64) -> SecureResult<u64> {
            a.checked_add(b).ok_or(SecurityError::ArithmeticOverflow)
        }

        pub fn sub(a: u64, b: u64) -> SecureResult<u64> {
            a.checked_sub(b).ok_or(SecurityError::ArithmeticOverflow)
        }

        pub fn mul(a: u64, b: u64) -> SecureResult<u64> {
            a.checked_mul(b).ok_or(SecurityError::ArithmeticOverflow)
        }
    }

    // =========================================================
    // 2. Access Control
    // =========================================================
    #[derive(Debug, Clone)]
    pub struct AccessControl {
        owner: AccountId,
        roles: HashMap<String, HashSet<AccountId>>,
    }

    impl AccessControl {
        pub fn new(owner: AccountId) -> Self {
            let mut roles = HashMap::new();

            let mut admin_set = HashSet::new();
            admin_set.insert(owner.clone());

            let mut pauser_set = HashSet::new();
            pauser_set.insert(owner.clone());

            let mut governor_set = HashSet::new();
            governor_set.insert(owner.clone());

            roles.insert("admin".to_string(), admin_set);
            roles.insert("pauser".to_string(), pauser_set);
            roles.insert("governor".to_string(), governor_set);

            Self { owner, roles }
        }

        pub fn owner(&self) -> &str {
            &self.owner
        }

        pub fn only_owner(&self, caller: &str) -> SecureResult<()> {
            if self.owner == caller {
                Ok(())
            } else {
                Err(SecurityError::Unauthorized)
            }
        }

        pub fn transfer_ownership(
            &mut self,
            caller: &str,
            new_owner: AccountId,
        ) -> SecureResult<()> {
            self.only_owner(caller)?;
            self.owner = new_owner.clone();

            self.roles
                .entry("admin".to_string())
                .or_default()
                .insert(new_owner.clone());

            self.roles
                .entry("pauser".to_string())
                .or_default()
                .insert(new_owner.clone());

            self.roles
                .entry("governor".to_string())
                .or_default()
                .insert(new_owner);

            Ok(())
        }

        pub fn grant_role(
            &mut self,
            admin: &str,
            role: &str,
            user: AccountId,
        ) -> SecureResult<()> {
            self.only_owner(admin)?;
            self.roles.entry(role.to_string()).or_default().insert(user);
            Ok(())
        }

        pub fn revoke_role(
            &mut self,
            admin: &str,
            role: &str,
            user: &str,
        ) -> SecureResult<()> {
            self.only_owner(admin)?;
            let members = self
                .roles
                .get_mut(role)
                .ok_or(SecurityError::RoleNotFound)?;

            members.remove(user);
            Ok(())
        }

        pub fn has_role(&self, user: &str, role: &str) -> bool {
            self.roles
                .get(role)
                .map(|members| members.contains(user))
                .unwrap_or(false)
        }
    }

    // =========================================================
    // 3. Reentrancy Guard (RAII-based)
    // =========================================================
    #[derive(Debug, Default)]
    pub struct ReentrancyGuard {
        locked: bool,
    }

    pub struct ReentrancyLock<'a> {
        guard: &'a mut ReentrancyGuard,
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

    // =========================================================
    // 4. Pausable
    // =========================================================
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

    // =========================================================
    // 5. Timelock
    // =========================================================
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

        pub fn propose_action(
            &mut self,
            id: ActionId,
            current_time: u64,
            data: String,
        ) {
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

        pub fn execute_action(
            &mut self,
            id: ActionId,
            current_time: u64,
        ) -> SecureResult<String> {
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

    // =========================================================
    // 6. SecureVault
    // =========================================================
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
                timelock: TimelockManager::new(86_400), // 24h
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

            // In a real blockchain environment, token/native transfer would occur here.

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

        // -----------------------------------------------------
        // Timelocked owner transfer
        // data format: "transfer_owner:<new_owner>"
        // -----------------------------------------------------
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
}

#[cfg(test)]
mod tests {
    use super::secure_shield::*;

    #[test]
    fn test_reentrancy_protection() {
        let mut vault = SecureVault::init("admin_user".to_string());

        let _lock = vault.guard.acquire().unwrap();

        let result = vault.deposit("user1".to_string(), 100);
        assert_eq!(result, Err(SecurityError::ReentrancyDetected));
    }

    #[test]
    fn test_access_control_owner() {
        let vault = SecureVault::init("admin_user".to_string());
        let result = vault.access.only_owner("hacker_user");
        assert_eq!(result, Err(SecurityError::Unauthorized));
    }

    #[test]
    fn test_safemath_overflow() {
        let res = SafeMath::add(u64::MAX, 1);
        assert_eq!(res, Err(SecurityError::ArithmeticOverflow));
    }

    #[test]
    fn test_pausable_logic() {
        let mut vault = SecureVault::init("admin_user".to_string());
        vault.emergency_stop("admin_user").unwrap();

        let result = vault.deposit("user1".to_string(), 50);
        assert_eq!(result, Err(SecurityError::ContractPaused));
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut vault = SecureVault::init("admin_user".to_string());

        vault.deposit("user1".to_string(), 200).unwrap();
        assert_eq!(vault.balance_of("user1"), 200);

        vault.withdraw("user1".to_string(), 80).unwrap();
        assert_eq!(vault.balance_of("user1"), 120);
    }

    #[test]
    fn test_withdraw_insufficient_balance() {
        let mut vault = SecureVault::init("admin_user".to_string());

        let result = vault.withdraw("user1".to_string(), 50);
        assert_eq!(result, Err(SecurityError::InsufficientBalance));
    }

    #[test]
    fn test_timelock_execution_too_early() {
        let mut vault = SecureVault::init("admin_user".to_string());

        vault
            .propose_owner_transfer("admin_user", 1, 1_000, "new_admin".to_string())
            .unwrap();

        let result = vault.execute_owner_transfer("admin_user", 1, 1_100);
        assert_eq!(result, Err(SecurityError::TimelockNotMet));
    }

    #[test]
    fn test_timelock_owner_transfer_success() {
        let mut vault = SecureVault::init("admin_user".to_string());

        let start_time = 1_000;
        vault
            .propose_owner_transfer("admin_user", 1, start_time, "new_admin".to_string())
            .unwrap();

        let execute_time = start_time + 86_400;
        vault
            .execute_owner_transfer("admin_user", 1, execute_time)
            .unwrap();

        assert_eq!(vault.access.owner(), "new_admin");
    }

    #[test]
    fn test_timelock_action_cannot_execute_twice() {
        let mut vault = SecureVault::init("admin_user".to_string());

        let start_time = 1_000;
        vault
            .propose_owner_transfer("admin_user", 7, start_time, "new_admin".to_string())
            .unwrap();

        let execute_time = start_time + 86_400;
        vault
            .execute_owner_transfer("admin_user", 7, execute_time)
            .unwrap();

        let result = vault.execute_owner_transfer("admin_user", 7, execute_time + 1);
        assert_eq!(result, Err(SecurityError::ActionAlreadyExecuted));
    }
}
