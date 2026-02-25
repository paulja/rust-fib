mod fib;

fn main() {
    let mut fib = fib::fibonacci();
    for _ in 0..30 {
        print!("{} ", fib());
    }
}
