use fib_core::fibonacci;
use num_format::{Locale, ToFormattedString};

pub fn run(n: u64) {
    let mut fib = fibonacci();
    let result = (0..n).map(|_| fib()).last().unwrap_or(0);
    println!("{}", result.to_formatted_string(&Locale::en));
}

#[cfg(test)]
mod tests {
    use fib_core::fibonacci;

    #[test]
    fn test_number_10() {
        // F(10) = 55
        let mut fib = fibonacci();
        let result = (0..10u64).map(|_| fib()).last().unwrap_or(0);
        assert_eq!(result, 55);
    }
}
