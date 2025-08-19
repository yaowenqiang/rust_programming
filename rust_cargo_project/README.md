# Rust Programming Examples

This repository contains various Rust programming examples organized using Cargo.

## Project Structure

- `src/lib.rs` - Library crate containing shared modules
- `src/mytypes.rs` - Shared types used across examples
- `src/bin/` - Directory containing individual binary examples:
  - `hello_world.rs` - Basic "Hello, World!" example
  - `variables_and_types.rs` - Variables and types in Rust
  - `flow_control.rs` - Control flow examples (if, match, loops)
  - `enum_demo.rs` - Enum usage examples
  - `borrow_check.rs` - Borrow checker examples
  - `arrays_demo.rs` - Arrays, vectors, and maps examples
  - `string_demo.rs` - String handling examples

## Running Examples

To run any example, use:

```bash
cargo run --bin <example_name>
```

For example:
```bash
cargo run --bin hello_world
cargo run --bin variables_and_types
```

## Building

To build all examples:
```bash
cargo build
```