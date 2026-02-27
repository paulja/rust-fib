# Contributing

This is primarily a personal learning project, but contributions are welcome.

## Getting Started

You'll need:

- [Rust](https://rustup.rs) (edition 2024, see `Cargo.toml` for the current version)
- [protoc](https://grpc.io/docs/protoc-installation/) — the protobuf compiler, required by `build.rs`
- [grpcurl](https://github.com/fullstorydev/grpcurl) — optional, for testing the gRPC server manually
- [Docker](https://www.docker.com) — optional, for running the containerised services

```sh
git clone https://github.com/paulja/fib
cd fib
cargo build
cargo test
```

## Making Changes

- Keep changes focused — one concern per PR
- Run `cargo test` and `cargo clippy` before submitting
- Follow existing code style; there is no separate formatter config beyond the defaults

## Submitting a Pull Request

1. Fork the repository and create a branch from `main`
2. Make your changes
3. Open a pull request with a clear description of what and why

## Reporting Issues

Open an issue on GitHub with enough detail to reproduce the problem.

## License

By contributing you agree that your contributions will be licensed under the [MIT License](LICENSE).
