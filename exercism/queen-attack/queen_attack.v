module main

fn can_queen_attack(white string, black string) !bool {
	if white.len < 2 {
		return error('${white} is not a valid square')
	}
	if black.len < 2 {
		return error('${black} is not a valid square')
	}
	wx := white[0]
	wy := white[1]
	bx := black[0]
	by := black[1]
	if wx < `a` || wx > `h` || wy < `1` || wy > `8` {
		return error('${white} is not a valid square')
	}
	if bx < `a` || bx > `h` || by < `1` || by > `8` {
		return error('${black} is not a valid square')
	}
	if wx == bx && wy == by {
		return error('queens on same square')
	}
	same_col := wx == bx
	same_row := wy == by
	same_diag := wx - wy == bx - by
	same_diag_rev := wx + wy == bx + by

	return same_col || same_row || same_diag || same_diag_rev
}
