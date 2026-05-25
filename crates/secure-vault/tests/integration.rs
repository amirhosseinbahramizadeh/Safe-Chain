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
