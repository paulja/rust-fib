package main

import (
	"context"
	_ "embed"
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
)

//go:embed fib_core.wasm
var wasmBytes []byte

// outPtr is the WASM linear memory offset used as the output buffer.
// Offset 0 is treated as null by Rust, so we start at 8.
const outPtr = 8

func main() {
	log.SetFlags(0)

	if len(os.Args) != 2 {
		log.Fatalf("usage: %s <n>", os.Args[0])
	}

	n, err := strconv.ParseUint(os.Args[1], 10, 64)
	if err != nil {
		log.Fatalf("invalid n: %v", err)
	}

	ctx := context.Background()

	rt := wazero.NewRuntime(ctx)
	defer rt.Close(ctx)

	mod, err := rt.InstantiateWithConfig(ctx, wasmBytes, wazero.NewModuleConfig())
	if err != nil {
		log.Fatalf("instantiate module: %v", err)
	}

	val, err := fibNumber(ctx, mod, n)
	if err != nil {
		log.Fatalf("fib_number: %v", err)
	}
	fmt.Printf("fib(%d) = %d\n", n, val)

	seq, err := fibSequence(ctx, mod, n)
	if err != nil {
		log.Fatalf("fib_sequence: %v", err)
	}
	fmt.Printf("sequence: %v\n", seq)
}

func fibNumber(ctx context.Context, mod api.Module, n uint64) (uint64, error) {
	results, err := mod.ExportedFunction("fib_number").Call(ctx, n, outPtr)
	if err != nil {
		return 0, err
	}
	if results[0] != 0 {
		return 0, fmt.Errorf("error code %d", results[0])
	}
	val, ok := mod.Memory().ReadUint64Le(outPtr)
	if !ok {
		return 0, fmt.Errorf("failed to read result from wasm memory")
	}
	return val, nil
}

func fibSequence(ctx context.Context, mod api.Module, n uint64) ([]uint64, error) {
	// Buffer starts at outPtr; each u64 is 8 bytes.
	results, err := mod.ExportedFunction("fib_sequence").Call(ctx, n, outPtr, n)
	if err != nil {
		return nil, err
	}
	if results[0] != 0 {
		return nil, fmt.Errorf("error code %d", results[0])
	}

	seq := make([]uint64, n)
	for i := range seq {
		val, ok := mod.Memory().ReadUint64Le(outPtr + uint32(i)*8)
		if !ok {
			return nil, fmt.Errorf("failed to read index %d from wasm memory", i)
		}
		seq[i] = val
	}
	return seq, nil
}
