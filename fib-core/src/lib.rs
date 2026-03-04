#[cfg(not(target_arch = "wasm32"))]
pub mod ffi;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub mod wasi;

pub fn fibonacci() -> impl FnMut() -> u64 {
    let (mut a, mut b) = (0, 1);
    move || {
        (a, b) = (b, a + b);
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = fibonacci();
        let got: Vec<u64> = (0..10).map(|_| fib()).collect();
        let want = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        assert_eq!(got, want);
    }
}
