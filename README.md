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

### gRPC API

Start the server with `fib grpc` (listens on port `50051`), then use `grpcurl`:

| RPC | Description |
|-----|-------------|
| `fib.FibonacciService/Number` | Returns the nth Fibonacci number |
| `fib.FibonacciService/Sequence` | Streams the first n Fibonacci numbers |

Valid range for `n`: `1–92`

```sh
grpcurl -plaintext -d '{"n": 10}' localhost:50051 fib.FibonacciService/Number
# {"n": "10", "value": "55"}

grpcurl -plaintext -d '{"n": 5}' localhost:50051 fib.FibonacciService/Sequence
# {"value": "1"}
# {"value": "1"}
# {"value": "2"}
# {"value": "3"}
# {"value": "5"}

# List available services (requires reflection)
grpcurl -plaintext localhost:50051 list
```

## Build & Run

```sh
cargo build --release
cargo run -- number 10
cargo run -- sequence 10
cargo run -- serve
cargo run -- grpc
```

## Docker

The `docker/` directory contains Dockerfiles for each server and a compose file to run both together.

| File | Description |
|------|-------------|
| `docker/Dockerfile.http` | HTTP server image (port `3000`) |
| `docker/Dockerfile.grpc` | gRPC server image (port `50051`) |
| `docker/compose.yaml` | Launches both services |

```sh
# Run both services
docker compose -f docker/compose.yaml up

# Run a single service
docker compose -f docker/compose.yaml up http
docker compose -f docker/compose.yaml up grpc
```

## Logging

Both servers emit structured JSON logs to stdout using the `tracing` ecosystem. Log level is controlled by the `RUST_LOG` environment variable (default: `info`).

```sh
RUST_LOG=debug cargo run -- serve
RUST_LOG=info,tower_http=debug cargo run -- grpc
```

To set the log level in Docker:

```yaml
# docker/compose.yaml
services:
  grpc:
    environment:
      - RUST_LOG=info,tower_http=debug
```

## Test

```sh
cargo test
```
