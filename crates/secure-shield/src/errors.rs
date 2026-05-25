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
