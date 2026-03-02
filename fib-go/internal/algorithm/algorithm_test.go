package algorithm

import (
	"testing"
)

var wantSequence = []uint64{1, 1, 2, 3, 5, 8, 13, 21, 34, 55}

func TestNumber(t *testing.T) {
	tests := []struct {
		name    string
		n       uint64
		want    uint64
		wantErr error
	}{
		{"first", 1, 1, nil},
		{"tenth", 10, 55, nil},
		{"max", 92, 7540113804746346429, nil},
		{"zero", 0, 0, ErrInvalidArgument},
		{"out of range", 93, 0, ErrInvalidArgument},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := Number(tt.n)
			if err != tt.wantErr {
				t.Fatalf("Number(%d) error = %v, want %v", tt.n, err, tt.wantErr)
			}
			if err == nil && got != tt.want {
				t.Errorf("Number(%d) = %d, want %d", tt.n, got, tt.want)
			}
		})
	}
}

func TestSequence(t *testing.T) {
	tests := []struct {
		name    string
		n       uint64
		want    []uint64
		wantErr error
	}{
		{"ten", 10, wantSequence, nil},
		{"one", 1, []uint64{1}, nil},
		{"zero", 0, nil, ErrInvalidArgument},
		{"out of range", 93, nil, ErrInvalidArgument},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := Sequence(tt.n)
			if err != tt.wantErr {
				t.Fatalf("Sequence(%d) error = %v, want %v", tt.n, err, tt.wantErr)
			}
			if err == nil {
				for i, v := range got {
					if v != tt.want[i] {
						t.Errorf("Sequence(%d)[%d] = %d, want %d", tt.n, i, v, tt.want[i])
					}
				}
			}
		})
	}
}

func TestRustNumber(t *testing.T) {
	tests := []struct {
		name    string
		n       uint64
		want    uint64
		wantErr error
	}{
		{"first", 1, 1, nil},
		{"tenth", 10, 55, nil},
		{"max", 92, 7540113804746346429, nil},
		{"zero", 0, 0, ErrInvalidArgument},
		{"out of range", 93, 0, ErrInvalidArgument},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := RustNumber(tt.n)
			if err != tt.wantErr {
				t.Fatalf("RustNumber(%d) error = %v, want %v", tt.n, err, tt.wantErr)
			}
			if err == nil && got != tt.want {
				t.Errorf("RustNumber(%d) = %d, want %d", tt.n, got, tt.want)
			}
		})
	}
}

func TestRustSequence(t *testing.T) {
	tests := []struct {
		name    string
		n       uint64
		want    []uint64
		wantErr error
	}{
		{"ten", 10, wantSequence, nil},
		{"one", 1, []uint64{1}, nil},
		{"zero", 0, nil, ErrInvalidArgument},
		{"out of range", 93, nil, ErrInvalidArgument},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := RustSequence(tt.n)
			if err != tt.wantErr {
				t.Fatalf("RustSequence(%d) error = %v, want %v", tt.n, err, tt.wantErr)
			}
			if err == nil {
				for i, v := range got {
					if v != tt.want[i] {
						t.Errorf("RustSequence(%d)[%d] = %d, want %d", tt.n, i, v, tt.want[i])
					}
				}
			}
		})
	}
}

func TestGoAndRustAgree(t *testing.T) {
	for n := uint64(1); n <= 92; n++ {
		goVal, err := Number(n)
		if err != nil {
			t.Fatalf("Number(%d): %v", n, err)
		}
		rustVal, err := RustNumber(n)
		if err != nil {
			t.Fatalf("RustNumber(%d): %v", n, err)
		}
		if goVal != rustVal {
			t.Errorf("n=%d: Go=%d, Rust=%d", n, goVal, rustVal)
		}
	}
}
