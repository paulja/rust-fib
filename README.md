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
| `serve` | Start the HTTP server |
| `grpc` | Start the gRPC server |

Valid range for `number` and `sequence`: `1–92` (max for u64 without overflow)

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
cargo run -- serve
cargo run -- grpc
```

## Test

```sh
cargo test
```
