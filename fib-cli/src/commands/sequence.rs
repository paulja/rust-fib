use fib_core::fibonacci;
use num_format::{Locale, ToFormattedString};

pub fn run(n: u64) {
    let mut fib = fibonacci();
    (0..n)
        .map(|_| fib())
        .for_each(|n| println!("{}", n.to_formatted_string(&Locale::en)));
}

#[cfg(test)]
mod tests {
    use fib_core::fibonacci;

    #[test]
    fn test_sequence_5() {
        let mut fib = fibonacci();
        let result: Vec<u64> = (0..5).map(|_| fib()).collect();
        assert_eq!(result, vec![1, 1, 2, 3, 5]);
    }
}
