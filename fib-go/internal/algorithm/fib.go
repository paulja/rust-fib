package algorithm

// Number returns the nth Fibonacci number using a pure Go implementation.
// n must be between 1 and 92 inclusive.
func Number(n uint64) (uint64, error) {
	if n == 0 || n > 92 {
		return 0, ErrInvalidArgument
	}
	fib := fibonacci()
	var value uint64
	for range n {
		value = fib()
	}
	return value, nil
}

// Sequence returns the first n Fibonacci numbers using a pure Go implementation.
// n must be between 1 and 92 inclusive.
func Sequence(n uint64) ([]uint64, error) {
	if n == 0 || n > 92 {
		return nil, ErrInvalidArgument
	}
	fib := fibonacci()
	out := make([]uint64, n)
	for i := range n {
		out[i] = fib()
	}
	return out, nil
}

// fibonacci returns a closure that yields successive Fibonacci numbers.
func fibonacci() func() uint64 {
	a, b := uint64(0), uint64(1)
	return func() uint64 {
		a, b = b, a+b
		return a
	}
}
