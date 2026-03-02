use std::panic::catch_unwind;

/// Returns the nth Fibonacci number via an output pointer.
///
/// Return codes: 0 = ok, 1 = invalid argument, 2 = panic
///
/// # Safety
///
/// `out` must be a valid, non-null, aligned pointer to a single `u64`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fib_number(n: u64, out: *mut u64) -> i32 {
    let result = catch_unwind(|| {
        if n == 0 || n > 92 {
            return 1;
        }
        let mut fib = crate::fibonacci();
        let value = (0..n).map(|_| fib()).last().unwrap();
        unsafe { *out = value };
        0
    });
    result.unwrap_or(2)
}

/// Fills `out[0..n]` with the first `n` Fibonacci numbers.
///
/// Return codes: 0 = ok, 1 = invalid argument, 2 = panic
///
/// # Safety
///
/// `out` must be a valid, non-null, aligned pointer to a buffer of at least `len` `u64` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fib_sequence(n: u64, out: *mut u64, len: u64) -> i32 {
    let result = catch_unwind(|| {
        if n == 0 || n > 92 || len < n {
            return 1;
        }
        let mut fib = crate::fibonacci();
        let slice = unsafe { std::slice::from_raw_parts_mut(out, len as usize) };
        for slot in slice.iter_mut().take(n as usize) {
            *slot = fib();
        }
        0
    });
    result.unwrap_or(2)
}
