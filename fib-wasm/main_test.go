package main

import (
	"context"
	"os"
	"testing"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
)

var testMod api.Module

func TestMain(m *testing.M) {
	ctx := context.Background()
	rt := wazero.NewRuntime(ctx)

	var err error
	testMod, err = rt.InstantiateWithConfig(ctx, wasmBytes, wazero.NewModuleConfig())
	if err != nil {
		panic("failed to instantiate wasm module: " + err.Error())
	}

	code := m.Run()

	rt.Close(ctx)
	os.Exit(code)
}

func TestFibNumber(t *testing.T) {
	ctx := context.Background()

	tests := []struct {
		n    uint64
		want uint64
	}{
		{1, 1},
		{2, 1},
		{10, 55},
		{92, 7540113804746346429},
	}

	for _, tt := range tests {
		got, err := fibNumber(ctx, testMod, tt.n)
		if err != nil {
			t.Errorf("fibNumber(%d): unexpected error: %v", tt.n, err)
			continue
		}
		if got != tt.want {
			t.Errorf("fibNumber(%d) = %d, want %d", tt.n, got, tt.want)
		}
	}
}

func TestFibNumberInvalid(t *testing.T) {
	ctx := context.Background()

	for _, n := range []uint64{0, 93, 100} {
		_, err := fibNumber(ctx, testMod, n)
		if err == nil {
			t.Errorf("fibNumber(%d): expected error, got nil", n)
		}
	}
}

func TestFibSequence(t *testing.T) {
	ctx := context.Background()

	got, err := fibSequence(ctx, testMod, 10)
	if err != nil {
		t.Fatalf("fibSequence(10): unexpected error: %v", err)
	}

	want := []uint64{1, 1, 2, 3, 5, 8, 13, 21, 34, 55}
	if len(got) != len(want) {
		t.Fatalf("fibSequence(10): got len %d, want %d", len(got), len(want))
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("fibSequence(10)[%d] = %d, want %d", i, got[i], want[i])
		}
	}
}

func TestFibSequenceInvalid(t *testing.T) {
	ctx := context.Background()

	for _, n := range []uint64{0, 93} {
		_, err := fibSequence(ctx, testMod, n)
		if err == nil {
			t.Errorf("fibSequence(%d): expected error, got nil", n)
		}
	}
}
