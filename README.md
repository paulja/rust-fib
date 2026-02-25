# fib

A command-line Fibonacci calculator written in Rust.

## Usage

```
fib <COMMAND> <N>
```

### Commands

| Command | Description |
|---------|-------------|
| `number N` | Calculate the Nth Fibonacci number |
| `sequence N` | Print the first N Fibonacci numbers |

Valid range: `1–92` (max for u64 without overflow)

### Examples

```sh
fib number 10
# 55

fib sequence 10
# 1
# 1
# 2
# 3
# 5
# 8
# 13
# 21
# 34
# 55
```

## Build & Run

```sh
cargo build --release
cargo run -- number 10
cargo run -- sequence 10
```

## Test

```sh
cargo test
```
