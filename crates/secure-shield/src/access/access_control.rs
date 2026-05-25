use std::collections::{HashMap, HashSet};

use crate::errors::SecurityError;
use crate::result::SecureResult;

pub type AccountId = String;

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
        let members = self.roles.get_mut(role).ok_or(SecurityError::RoleNotFound)?;
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
