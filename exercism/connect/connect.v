module main

import strings

fn dfs(mat []string, starti int, startj int, targeti int, targetj int) bool {
	c := mat[starti][startj]
	if c == `.` {
		return false
	}
	println('finding way from ${starti} ${startj} (${c}) to ${targeti} ${targetj}')

	mut vis := [][]bool{len: mat.len, init: []bool{len: mat[0].len}}
	mut q := [][2]int{}
	q << [starti, startj]!

	valid_moves := [
		[-1, 0],
		[1, 0],
		[0, -1],
		[0, 1],
		[-1, 1],
		[1, -1],
	]

	for q.len > 0 {
		curr := q[0]
		i, j := curr[0], curr[1]
		q = q[1..]

		if vis[i][j] {
			continue
		}
		vis[i][j] = true

		if (targeti == -1 || targeti == i) && (targetj == -1 || targetj == j) {
			return true
		}

		for d in valid_moves {
			ni := i + d[0]
			nj := j + d[1]
			if ni < 0 || ni >= mat.len || nj < 0 || nj >= mat[ni].len {
				continue
			}
			if mat[ni][nj] == c {
				q << [ni, nj]!
			}
		}
	}
	return false
}

fn winner(board []string) ?rune {
	n := board.len
	mut mat := []string{len: n}

	for i, l in board {
		mat[i] = l.replace(' ', '')
	}

	println('mat: \n${mat.join('\n')}')
	m := mat[0].len

	for i := 0; i < n; i++ {
		if dfs(mat, i, 0, -1, m - 1) {
			println('returning ${mat[i][0]}')
			return mat[i][0]
		}
	}

	for j := 0; j < m; j++ {
		if dfs(mat, 0, j, n - 1, -1) {
			println('returning ${mat[0][j]}')
			return mat[0][j]
		}
	}
	println('returning none')
	return none
}
