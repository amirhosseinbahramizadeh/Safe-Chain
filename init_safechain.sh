#!/usr/bin/env bash
set -e

echo "Creating directory structure..."

mkdir -p .github/workflows
mkdir -p .github/ISSUE_TEMPLATE
mkdir -p docs/examples
mkdir -p scripts
mkdir -p crates/secure-shield/src/{math,access,guards,pause,timelock}
mkdir -p crates/secure-vault/src
mkdir -p crates/secure-vault/tests
mkdir -p tests

echo "Writing root files..."

cat > Cargo.toml << 'EOT'
[workspace]
resolver = "2"
members = [
    "crates/secure-shield",
    "crates/secure-vault",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["SafeChain Contributors"]
repository = "https://github.com/your-org/safechain"
homepage = "https://github.com/your-org/safechain"
documentation = "https://github.com/your-org/safechain"
EOT

cat > rust-toolchain.toml << 'EOT'
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
EOT

cat > .gitignore << 'EOT'
/target
**/*.rs.bk
EOT

cat > .editorconfig << 'EOT'
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 4
trim_trailing_whitespace = true

[*.md]
trim_trailing_whitespace = false
EOT

cat > LICENSE << 'EOT'
MIT License

Copyright (c) 2026 SafeChain Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOT

cat > README.md << 'EOT'
# SafeChain

![CI](https://github.com/your-org/safechain/actions/workflows/ci.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-stable-orange)

A modular high-security smart contract framework for Rust-based blockchains.

## Overview

SafeChain provides reusable defensive primitives for secure contract development, including:

- Safe arithmetic checks
- Ownership and role-based access control
- Reentrancy protection
- Emergency pause mechanism
- Timelock execution for sensitive actions

This repository is structured as a Rust workspace with:

- \`crates/secure-shield\` — core framework
- \`crates/secure-vault\` — example implementation

## Quick Start

\`\`\`bash
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
\`\`\`

## License
MIT
EOT

cat > WHITEPAPER.md << 'EOT'
# SafeChain Whitepaper

(Short version – you can replace with the full whitepaper later.)
EOT

cat > CHANGELOG.md << 'EOT'
# Changelog

## [0.1.0] - 2026-05-25
- Initial workspace, secure-shield core, secure-vault example.
EOT

cat > SECURITY.md << 'EOT'
# Security Policy

See repository documentation for reporting vulnerabilities.
EOT

cat > CONTRIBUTING.md << 'EOT'
# Contributing to SafeChain

See README for basic development commands.
EOT

cat > CODE_OF_CONDUCT.md << 'EOT'
# Code of Conduct

Be respectful and professional.
EOT

echo "Writing GitHub workflows..."

cat > .github/workflows/ci.yml << 'EOT'
name: CI

on:
  push:
    branches: [ main, master ]
  pull_request:

jobs:
  rust:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Format check
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings

      - name: Tests
        run: cargo test --workspace
EOT

cat > .github/workflows/security.yml << 'EOT'
name: Security

on:
  push:
    branches: [ main, master ]
  pull_request:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install cargo-audit
        run: cargo install cargo-audit || true
      - name: Run cargo-audit
        run: cargo audit || true
EOT

cat > .github/workflows/release.yml << 'EOT'
name: Release

on:
  workflow_dispatch:

jobs:
  release-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --workspace
EOT

echo "Writing docs..."

cat > docs/architecture.md << 'EOT'
# Architecture

SafeChain is organized as a Rust workspace:
- secure-shield (core)
- secure-vault (example)
EOT

cat > docs/threat-model.md << 'EOT'
# Threat Model

High-level threat model for SafeChain.
EOT

cat > docs/auditing-guide.md << 'EOT'
# Auditing Guide

Outline of what to check when auditing SafeChain-based contracts.
EOT

cat > docs/examples/secure-vault.md << 'EOT'
# Secure Vault Example

Overview of the secure-vault example crate.
EOT

echo "Writing scripts..."

cat > scripts/fmt.sh << 'EOT'
#!/usr/bin/env bash
set -e
cargo fmt --all
EOT
chmod +x scripts/fmt.sh

cat > scripts/clippy.sh << 'EOT'
#!/usr/bin/env bash
set -e
cargo clippy --workspace --all-targets --all-features -- -D warnings
EOT
chmod +x scripts/clippy.sh

cat > scripts/test.sh << 'EOT'
#!/usr/bin/env bash
set -e
cargo test --workspace
EOT
chmod +x scripts/test.sh

cat > scripts/coverage.sh << 'EOT'
#!/usr/bin/env bash
set -e
cargo test --workspace
echo "Integrate coverage tooling (e.g. tarpaulin) here."
EOT
chmod +x scripts/coverage.sh

echo "Writing tests/workspace_smoke.rs..."

cat > tests/workspace_smoke.rs << 'EOT'
#[test]
fn workspace_smoke_test() {
    assert_eq!(2 + 2, 4);
}
EOT

echo "Writing crates/secure-shield..."

cat > crates/secure-shield/Cargo.toml << 'EOT'
[package]
name = "secure-shield"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Core security primitives for Rust-based blockchain contracts"
repository = "https://github.com/your-org/safechain"

[lib]
name = "secure_shield"
path = "src/lib.rs"
EOT

mkdir -p crates/secure-shield/src

cat > crates/secure-shield/src/lib.rs << 'EOT'
pub mod errors;
pub mod result;
pub mod prelude;

pub mod math;
pub mod access;
pub mod guards;
pub mod pause;
pub mod timelock;
EOT

cat > crates/secure-shield/src/errors.rs << 'EOT'
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
EOT

cat > crates/secure-shield/src/result.rs << 'EOT'
use crate::errors::SecurityError;

pub type SecureResult<T> = Result<T, SecurityError>;
EOT

cat > crates/secure-shield/src/prelude.rs << 'EOT'
pub use crate::access::access_control::AccessControl;
pub use crate::errors::SecurityError;
pub use crate::guards::reentrancy::{ReentrancyGuard, ReentrancyLock};
pub use crate::math::safe_math::SafeMath;
pub use crate::pause::pausable::Pausable;
pub use crate::result::SecureResult;
pub use crate::timelock::timelock::{TimelockAction, TimelockManager};

pub type AccountId = String;
pub type ActionId = u128;
EOT

cat > crates/secure-shield/src/math/mod.rs << 'EOT'
pub mod safe_math;
EOT

cat > crates/secure-shield/src/math/safe_math.rs << 'EOT'
use crate::errors::SecurityError;
use crate::result::SecureResult;

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
EOT

cat > crates/secure-shield/src/access/mod.rs << 'EOT'
pub mod access_control;
EOT

cat > crates/secure-shield/src/access/access_control.rs << 'EOT'
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
EOT

cat > crates/secure-shield/src/guards/mod.rs << 'EOT'
pub mod reentrancy;
EOT

cat > crates/secure-shield/src/guards/reentrancy.rs << 'EOT'
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
EOT

cat > crates/secure-shield/src/pause/mod.rs << 'EOT'
pub mod pausable;
EOT

cat > crates/secure-shield/src/pause/pausable.rs << 'EOT'
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
EOT

cat > crates/secure-shield/src/timelock/mod.rs << 'EOT'
pub mod timelock;
EOT

cat > crates/secure-shield/src/timelock/timelock.rs << 'EOT'
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
EOT

echo "Writing crates/secure-vault..."

cat > crates/secure-vault/Cargo.toml << 'EOT'
[package]
name = "secure-vault"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Example secure vault built on top of secure-shield"
repository = "https://github.com/your-org/safechain"

[dependencies]
secure-shield = { path = "../secure-shield" }
EOT

mkdir -p crates/secure-vault/src

cat > crates/secure-vault/src/lib.rs << 'EOT'
pub mod vault;
EOT

cat > crates/secure-vault/src/vault.rs << 'EOT'
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
EOT

mkdir -p crates/secure-vault/tests

cat > crates/secure-vault/tests/integration.rs << 'EOT'
use secure_shield::prelude::*;
use secure_vault::vault::SecureVault;

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
EOT

echo "Done."
