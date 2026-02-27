use clap::{Parser, Subcommand};

mod commands;
mod fib;

/// Fibonacci number calculator.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Calculate the nth Fibonacci number (max 92).
    Number {
        #[arg(value_parser = clap::value_parser!(u64).range(1..=92))]
        number: u64,
    },
    /// Calculate the Fibonacci sequence up to n terms (max 92).
    Sequence {
        #[arg(value_parser = clap::value_parser!(u64).range(1..=92))]
        sequence: u64,
    },
    /// Start the HTTP server.
    Serve,
    /// Start the gRPC server.
    Grpc,
}

fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("info,tower_http=debug")
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .init();

    let args = Cli::parse();
    match args.command {
        Command::Number { number } => commands::number::run(number),
        Command::Sequence { sequence } => commands::sequence::run(sequence),
        Command::Serve => commands::serve::run(),
        Command::Grpc => commands::grpc::run(),
    }
}
