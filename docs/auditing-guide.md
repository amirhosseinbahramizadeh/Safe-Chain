# Auditing Guide

## Audit Priorities

### 1. Access Boundaries
Verify all privileged functions enforce:
- owner checks
- role checks
- expected authorization model

### 2. State Safety
Check:
- pause gating
- balance mutation ordering
- timelock execution state transitions

### 3. Reentrancy Controls
Review:
- lock acquisition points
- automatic lock release behavior
- nested entry failure mode

### 4. Timelock Correctness
Verify:
- delay enforcement
- replay prevention
- action parsing safety

### 5. Error Semantics
Ensure errors are meaningful and map correctly to failure classes.

## Suggested Test Classes
- positive functional tests
- negative authorization tests
- overflow tests
- replay tests
- pause-state tests
- ownership transfer tests
