# Threat Model

## Assets
- user balances
- ownership privileges
- administrative roles
- queued timelock actions

## Threats
- unauthorized state changes
- reentrant execution
- arithmetic overflow/underflow
- malicious or accidental admin actions
- repeated execution of sensitive actions

## Mitigations
- explicit owner/role checks
- checked arithmetic
- reentrancy lock
- pausable emergency stop
- timelocked admin actions
- one-time execution enforcement

## Assumptions
- caller identity is provided correctly by the surrounding environment
- storage integrity is preserved by the host chain/runtime
- time source is trustworthy enough for timelock enforcement

## Out of Scope
- private key theft
- validator collusion
- chain-level rollback attacks
- bridge compromise
