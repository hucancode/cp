package grains

Error :: enum {
	None,
	SquareOutOfRange,
	InvalidSquare,
}

square :: proc(n: int) -> (u64, Error) {
	if n < 1 || n > 64 {
		return 0, Error.SquareOutOfRange
	}
	n := uint(n - 1)
	return 1 << n, Error.None
}

total :: proc() -> (u64, Error) {
	return (1 << 64) - 1, Error.None
}
