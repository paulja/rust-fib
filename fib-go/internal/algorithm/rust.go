package algorithm

/*
#cgo LDFLAGS: -L../../../target/release -lfib_core -lm
#include "../../../fib-core/fib_core.h"
*/
import "C"

const (
	ok     = 0
	badArg = 1
)

// RustNumber returns the nth Fibonacci number by calling the Rust fib-core library via CGo.
// n must be between 1 and 92 inclusive.
func RustNumber(n uint64) (uint64, error) {
	if n == 0 || n > 92 {
		return 0, ErrInvalidArgument
	}
	var out C.uint64_t
	switch C.fib_number(C.uint64_t(n), &out) {
	case ok:
		return uint64(out), nil
	case badArg:
		return 0, ErrInvalidArgument
	default:
		return 0, ErrPanic
	}
}

// RustSequence returns the first n Fibonacci numbers by calling the Rust fib-core library via CGo.
// n must be between 1 and 92 inclusive.
func RustSequence(n uint64) ([]uint64, error) {
	if n == 0 || n > 92 {
		return nil, ErrInvalidArgument
	}
	buf := make([]C.uint64_t, n)
	switch C.fib_sequence(C.uint64_t(n), &buf[0], C.uint64_t(n)) {
	case ok:
		out := make([]uint64, n)
		for i, v := range buf {
			out[i] = uint64(v)
		}
		return out, nil
	case badArg:
		return nil, ErrInvalidArgument
	default:
		return nil, ErrPanic
	}
}
