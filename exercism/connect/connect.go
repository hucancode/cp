package connect

import (
	"fmt"
	"strings"
)

func dfs(mat []string, i, j, targeti, targetj int) bool {
	c := mat[i][j]
	if c == '.' {
		return false
	}
	fmt.Printf("finding way from %d %d (%c) to %d %d\n", i, j, c, targeti, targetj)
	vis := make([][]bool, len(mat))
	for i := range vis {
		vis[i] = make([]bool, len(mat[i]))
	}
	q := make([][2]int, 0)
	q = append(q, [2]int{i, j})
	validMoves := [][2]int{
		{-1, 0}, {1, 0},
		{0, -1}, {0, 1},
		{-1, 1}, {1, -1},
	}
	for len(q) > 0 {
		i, j := q[0][0], q[0][1]
		q = q[1:]
		if vis[i][j] {
			continue
		}
		vis[i][j] = true
		if (targeti == -1 || targeti == i) && (targetj == -1 || targetj == j) {
			return true
		}
		for _, d := range validMoves {
			ni, nj := i+d[0], j+d[1]
			if ni < 0 || ni >= len(mat) || nj < 0 || nj >= len(mat[ni]) {
				continue
			}
			if mat[ni][nj] == c {
				q = append(q, [2]int{ni, nj})
			}
		}
	}
	return false
}
func ResultOf(lines []string) (string, error) {
	n := len(lines)
	mat := make([]string, n)
	for i, l := range lines {
		mat[i] = strings.TrimSpace(l)
	}
	fmt.Printf("mat: \n%s\n", strings.Join(mat, "\n"))
	m := len(mat[0])
	for i := 0; i < n; i++ {
		if dfs(mat, i, 0, -1, m-1) {
			return string(mat[i][0]), nil
		}
	}
	for j := 0; j < m; j++ {
		if dfs(mat, 0, j, n-1, -1) {
			return string(mat[0][j]), nil
		}
	}
	return "", nil
}
