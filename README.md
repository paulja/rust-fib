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

### HTTP API

Start the server with `fib serve`, then use the following endpoints:

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/fib/{n}` | Returns the nth Fibonacci number |
| `GET` | `/fib/sequence/{n}` | Returns the first n Fibonacci numbers |

Valid range for `n`: `1–92`

```sh
curl http://localhost:3000/fib/10
# {"n":10,"value":55}

curl http://localhost:3000/fib/sequence/5
# {"n":5,"values":[1,1,2,3,5]}

curl http://localhost:3000/fib/100
# {"error":"n must be between 1 and 92"}
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
