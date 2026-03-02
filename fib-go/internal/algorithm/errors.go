package algorithm

import "errors"

var (
	ErrInvalidArgument = errors.New("n must be between 1 and 92 inclusive")
	ErrPanic           = errors.New("fibonacci computation panicked")
)
