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
