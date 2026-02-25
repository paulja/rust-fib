use clap::{Parser, Subcommand};
use num_format::{Locale, ToFormattedString};

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
}

fn main() {
    let args = Cli::parse();
    let mut fib = fib::fibonacci();
    match args.command {
        Command::Number { number } => {
            let result = (0..number).map(|_| fib()).last().unwrap_or(0);
            println!("{}", result.to_formatted_string(&Locale::en));
        }
        Command::Sequence { sequence } => {
            (0..sequence)
                .map(|_| fib())
                .for_each(|n| println!("{}", n.to_formatted_string(&Locale::en)));
        }
    }
}
