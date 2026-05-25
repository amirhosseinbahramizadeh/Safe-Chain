use crate::errors::SecurityError;

pub type SecureResult<T> = Result<T, SecurityError>;
