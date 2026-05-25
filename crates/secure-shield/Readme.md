# secure-shield

Core security primitives for Rust-based blockchain applications.

## Modules
- SafeMath
- AccessControl
- ReentrancyGuard
- Pausable
- TimelockManager

## Usage
Import the prelude:
```rust
use secure_shield::prelude::*;


---

## `crates/secure-shield/src/lib.rs`
```rust
pub mod errors;
pub mod result;
pub mod prelude;

pub mod math;
pub mod access;
pub mod guards;
pub mod pause;
pub mod timelock;
