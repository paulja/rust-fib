# fib

A Fibonacci calculator written in Rust, with an HTTP server, gRPC server, and a Go CLI that calls the Rust core via FFI.

## Project structure

```
fib/
  fib-core/       ← Fibonacci logic; compiled as a Rust library and C-compatible shared/static lib
  fib-cli/        ← Rust CLI binary
  fib-http/       ← HTTP server (Axum, port 3000)
  fib-grpc/       ← gRPC server (Tonic, port 50051)
  fib-go/         ← Go CLI that calls fib-core via CGo FFI
  docker/         ← Dockerfiles and compose file
```

## Rust CLI

```sh
cargo build --release
cargo run -- number 10
cargo run -- sequence 10
cargo run -- serve
cargo run -- grpc
```

### Commands

| Command | Description |
|---------|-------------|
| `number N` | Calculate the Nth Fibonacci number |
| `sequence N` | Print the first N Fibonacci numbers |
| `serve` | Start the HTTP server |
| `grpc` | Start the gRPC server |

Valid range for `number` and `sequence`: `1–92` (max for u64 without overflow)

```sh
fib number 10
# 55

fib sequence 10
# 1 1 2 3 5 8 13 21 34 55
```

## Go CLI (FFI)

The `fib-go` directory contains a Go CLI that calls `fib-core` via CGo. It supports two implementations for comparison: the native Go implementation and the Rust implementation via FFI.

### Prerequisites

- Go 1.26+
- A C compiler (gcc or clang)
- The `fib-core` release build: `cargo build --release -p fib-core`

### Usage

```sh
go -C fib-go run ./cmd/main.go <n> <native|rust>
```

```sh
go -C fib-go run ./cmd/main.go 10 native
# [1 1 2 3 5 8 13 21 34 55]  (3.4µs, native)

go -C fib-go run ./cmd/main.go 10 rust
# [1 1 2 3 5 8 13 21 34 55]  (668ns, rust)
```

### Tests

```sh
go -C fib-go test ./...
```

### Container

```sh
docker build -f docker/Dockerfile.fib-go . -t fib-go
docker run -it fib-go
```

This drops you into a shell. From there:

```sh
fib-go 10 native
fib-go 10 rust
```

## HTTP API

Start the server with `fib serve` or via Docker, then:

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/fib/{n}` | Returns the nth Fibonacci number |
| `GET` | `/fib/sequence/{n}` | Returns the first n Fibonacci numbers |

```sh
curl http://localhost:3000/fib/10
# {"n":10,"value":55}

curl http://localhost:3000/fib/sequence/5
# {"n":5,"values":[1,1,2,3,5]}
```

## gRPC API

Start the server with `fib grpc` or via Docker (listens on port `50051`), then use `grpcurl`:

| RPC | Description |
|-----|-------------|
| `fib.FibonacciService/Number` | Returns the nth Fibonacci number |
| `fib.FibonacciService/Sequence` | Streams the first n Fibonacci numbers |

```sh
grpcurl -plaintext -d '{"n": 10}' localhost:50051 fib.FibonacciService/Number
# {"n": "10", "value": "55"}

grpcurl -plaintext -d '{"n": 5}' localhost:50051 fib.FibonacciService/Sequence
# {"value": "1"} {"value": "1"} {"value": "2"} {"value": "3"} {"value": "5"}
```

## Docker

| File | Description |
|------|-------------|
| `docker/Dockerfile.http` | HTTP server image (port `3000`) |
| `docker/Dockerfile.grpc` | gRPC server image (port `50051`) |
| `docker/Dockerfile.fib-go` | Go FFI CLI interactive image |
| `docker/compose.yaml` | Launches HTTP and gRPC services |

```sh
# Run both Rust services
docker compose -f docker/compose.yaml up

# Run a single service
docker compose -f docker/compose.yaml up http
docker compose -f docker/compose.yaml up grpc

# Run the Go FFI CLI interactively
docker build -f docker/Dockerfile.fib-go . -t fib-go
docker run -it fib-go
```

## Logging

Both Rust servers emit structured JSON logs to stdout via the `tracing` ecosystem. Log level is controlled by `RUST_LOG` (default: `info`).

```sh
RUST_LOG=debug cargo run -- serve
RUST_LOG=info,tower_http=debug cargo run -- grpc
```

## Test

```sh
# Rust
cargo test

# Go
go -C fib-go test ./...
```
