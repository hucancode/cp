package connect

import (
	"strings"
	"testing"
)

// This is an auto-generated file. Do not change it manually. Run the generator to update the file.
// See https://github.com/exercism/go#synchronizing-tests-and-instructions
// Source: exercism/problem-specifications
// Commit: 42dd0ce Remove version (#1678)

var testCases = []struct {
	description string
	board       []string
	expected    string
}{
	{
		description: "an empty board has no winner",
		board: []string{
			". . . . .",
			" . . . . .",
			"  . . . . .",
			"   . . . . .",
			"    . . . . .",
		},
		expected: "",
	},
	{
		description: "X can win on a 1x1 board",
		board: []string{
			"X",
		},
		expected: "X",
	},
	{
		description: "O can win on a 1x1 board",
		board: []string{
			"O",
		},
		expected: "O",
	},
	{
		description: "only edges does not make a winner",
		board: []string{
			"O O O X",
			" X . . X",
			"  X . . X",
			"   X O O O",
		},
		expected: "",
	},
	{
		description: "illegal diagonal does not make a winner",
		board: []string{
			"X O . .",
			" O X X X",
			"  O X O .",
			"   . O X .",
			"    X X O O",
		},
		expected: "",
	},
	{
		description: "nobody wins crossing adjacent angles",
		board: []string{
			"X . . .",
			" . X O .",
			"  O . X O",
			"   . O . X",
			"    . . O .",
		},
		expected: "",
	},
	{
		description: "X wins crossing from left to right",
		board: []string{
			". O . .",
			" O X X X",
			"  O X O .",
			"   X X O X",
			"    . O X .",
		},
		expected: "X",
	},
	{
		description: "O wins crossing from top to bottom",
		board: []string{
			". O . .",
			" O X X X",
			"  O O O .",
			"   X X O X",
			"    . O X .",
		},
		expected: "O",
	},
	{
		description: "X wins using a convoluted path",
		board: []string{
			". X X . .",
			" X . X . X",
			"  . X . X .",
			"   . X X . .",
			"    O O O O O",
		},
		expected: "X",
	},
	{
		description: "X wins using a spiral path",
		board: []string{
			"O X X X X X X X X",
			" O X O O O O O O O",
			"  O X O X X X X X O",
			"   O X O X O O O X O",
			"    O X O X X X O X O",
			"     O X O O O X O X O",
			"      O X X X X X O X O",
			"       O O O O O O O X O",
			"        X X X X X X X X O",
		},
		expected: "X",
	},
}

// Simply strip the spaces of all the strings to get a canonical
// input. The spaces are only for readability of the tests.

func prepare(lines []string) []string {
	newLines := make([]string, len(lines))
	for i, l := range lines {
		newLines[i] = strings.ReplaceAll(l, " ", "")
	}
	return newLines
}

func TestResultOf(t *testing.T) {
	for _, tc := range testCases {
		t.Run(tc.description, func(t *testing.T) {
			actual, err := ResultOf(prepare(tc.board))
			// We don't expect errors for any of the test cases
			if err != nil {
				t.Errorf("ResultOf() returned error %v\nboard: \n%s\nwant: %q", err, strings.Join(tc.board, "\n"), tc.expected)
			} else if actual != tc.expected {
				t.Errorf("ResultOf() returned wrong result \nboard: \n%s\ngot: %q\nwant: %q", strings.Join(tc.board, "\n"), actual, tc.expected)
			}
		})
	}
}

func BenchmarkResultOf(b *testing.B) {
	if testing.Short() {
		b.Skip("skipping benchmark in short mode.")
	}
	b.StopTimer()
	for _, tt := range testCases {
		board := prepare(tt.board)
		b.StartTimer()
		for i := 0; i < b.N; i++ {
			ResultOf(board)
		}
		b.StopTimer()
	}

}
