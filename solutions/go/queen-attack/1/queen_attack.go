package queenattack

import (
	"fmt"
)

func CanQueenAttack(whitePosition, blackPosition string) (bool, error) {
	var wx, wy, bx, by uint8
	fmt.Sscanf(whitePosition, "%c%d", &wx, &wy)
	fmt.Sscanf(blackPosition, "%c%d", &bx, &by)
	if wx < 'a' || wx > 'h' || bx < 'a' || bx > 'h' || wy < 1 || wy > 8 || by < 1 || by > 8 {
		return false, fmt.Errorf("invalid position")
	}
	if wx == bx && wy == by {
		return false, fmt.Errorf("same position")
	}
	sameCol := wx == bx
	sameRow := wy == by
	sameDiag := wx-wy == bx-by
	sameDiagRev := wx+wy == bx+by
	return sameCol || sameRow || sameDiag || sameDiagRev, nil
}
