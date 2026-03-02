package main

import (
	"fmt"
	"os"
	"strconv"
	"time"

	"fib-go/internal/algorithm"
)

func main() {
	if len(os.Args) != 3 {
		fmt.Fprintln(os.Stderr, "usage: fib-go <n> <native|rust>")
		os.Exit(1)
	}

	n, err := strconv.ParseUint(os.Args[1], 10, 64)
	if err != nil || n == 0 || n > 92 {
		fmt.Fprintln(os.Stderr, "error: n must be an integer between 1 and 92")
		os.Exit(1)
	}

	impl := os.Args[2]

	var seq []uint64
	start := time.Now()

	switch impl {
	case "native":
		seq, err = algorithm.Sequence(n)
	case "rust":
		seq, err = algorithm.RustSequence(n)
	default:
		fmt.Fprintln(os.Stderr, "error: implementation must be 'native' or 'rust'")
		os.Exit(1)
	}

	duration := time.Since(start)

	if err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("%v  (%v, %s)\n", seq, duration, impl)
}
