# Architecture

## Overview

SafeChain is organized as a Rust workspace with a clean separation between:

1. Core security primitives (`secure-shield`)
2. Example implementation (`secure-vault`)

## Core Layers

### Error Layer
Defines a shared error taxonomy for all modules.

### Utility Layer
Contains checked arithmetic helpers.

### Control Layer
Contains:
- access control
- pause control
- reentrancy protection

### Governance Delay Layer
Contains timelocked action scheduling and execution.

## Design Principles

- minimal shared state
- explicit failure conditions
- modular composition
- low cognitive overhead
- testability first

## Example Flow

1. User calls `deposit`
2. Pause state is checked
3. Reentrancy lock is acquired
4. Balance is updated using checked arithmetic
5. Lock is released automatically on scope exit
