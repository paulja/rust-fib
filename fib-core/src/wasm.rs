use wasm_bindgen::prelude::*;

/// Returns the nth Fibonacci number.
///
/// Panics if `n` is 0 or greater than 92 — in WASM a panic becomes a JS exception.
#[wasm_bindgen]
pub fn fib_number(n: u64) -> u64 {
    if n == 0 || n > 92 {
        panic!("n must be between 1 and 92");
    }
    let mut fib = crate::fibonacci();
    (0..n).map(|_| fib()).last().unwrap()
}

/// Returns the first `n` Fibonacci numbers as a `BigUint64Array` in JS.
///
/// Panics if `n` is 0 or greater than 92 — in WASM a panic becomes a JS exception.
#[wasm_bindgen]
pub fn fib_sequence(n: u64) -> Vec<u64> {
    if n == 0 || n > 92 {
        panic!("n must be between 1 and 92");
    }
    let mut fib = crate::fibonacci();
    (0..n).map(|_| fib()).collect()
}
