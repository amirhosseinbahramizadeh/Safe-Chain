# Contributing to SafeChain

Thank you for your interest in contributing.

## Development Setup
```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace

## Pull Requests
- Create a feature branch
- Add tests for any behavior changes
- Keep commits clean and focused
- Update docs if necessary

## Commit Style
Recommended:
- feat: add timelock validation
- fix: prevent duplicate action execution
- docs: update architecture guide


---

## `SECURITY.md`
```md
# Security Policy

## Supported Versions
This project is currently in MVP stage. Security fixes will be applied to the latest version.

## Reporting a Vulnerability
Please do not disclose security vulnerabilities publicly before coordinated review.

Send a report including:
- Description
- Impact
- Reproduction steps
- Suggested mitigation

Recommended private contact:
- security@safechain.local

## Scope
In scope:
- Access control bypass
- Reentrancy vulnerabilities
- Timelock bypass
- Arithmetic issues
- State consistency issues

Out of scope:
- Feature requests
- Non-security code style issues
