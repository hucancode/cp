package zebra

import "fmt"

type Solution struct {
	DrinksWater string
	OwnsZebra   string
}

const NATION_IDX = 0
const COLOR_IDX = 1
const PET_IDX = 2
const DRINK_IDX = 3
const SMOKE_IDX = 4

const NATION_EN = 0
const NATION_SP = 1
const NATION_UKR = 2
const NATION_NOR = 3
const NATION_JP = 4

var NATIONS = []string{
	"Englishman",
	"Spaniard",
	"Ukrainian",
	"Norwegian",
	"Japanese",
}

const COLOR_RED = 0
const COLOR_GREEN = 1
const COLOR_IVORY = 2
const COLOR_YELLOW = 3
const COLOR_BLUE = 4

const PET_DOG = 0
const PET_SNAILS = 1
const PET_FOX = 2
const PET_HORSE = 3
const PET_ZEBRA = 4

const DRINK_TEA = 0
const DRINK_COFFEE = 1
const DRINK_MILK = 2
const DRINK_ORANGE = 3
const DRINK_WATER = 4

const SMOKE_OLDGOLD = 0
const SMOKE_KOOLS = 1
const SMOKE_CHESTERFIELD = 2
const SMOKE_LUCKYSTRIKE = 3
const SMOKE_PARLIAMENT = 4

func compilant(houses [5][5]int) bool {
	// fmt.Printf("checking compilant %v\n", houses)
	// The Englishman lives in the red house
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_EN && houses[i][COLOR_IDX] != COLOR_RED {
			return false
		}
	}
	// The Spaniard owns the dog
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_SP && houses[i][PET_IDX] != PET_DOG {
			return false
		}
	}
	// Coffee is drunk in the green house
	for i := 0; i < 5; i++ {
		if houses[i][DRINK_IDX] == DRINK_COFFEE && houses[i][COLOR_IDX] != COLOR_GREEN {
			return false
		}
	}
	// The Ukrainian drinks tea
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_UKR && houses[i][DRINK_IDX] != DRINK_TEA {
			return false
		}
	}
	// The green house is immediately to the right of the ivory house
	if houses[0][COLOR_IDX] == COLOR_GREEN {
		return false
	}
	for i := 1; i < 5; i++ {
		if houses[i][COLOR_IDX] == COLOR_GREEN {
			if i == 0 || houses[i-1][COLOR_IDX] != COLOR_IVORY {
				return false
			}
		}
	}
	// The Old Gold smoker owns snails
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_OLDGOLD && houses[i][PET_IDX] != PET_SNAILS {
			return false
		}
	}
	// Kools are smoked in the yellow house
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_KOOLS && houses[i][COLOR_IDX] != COLOR_YELLOW {
			return false
		}
	}
	// Milk is drunk in the middle house
	if houses[2][DRINK_IDX] != DRINK_MILK {
		return false
	}
	// The Norwegian lives in the first house
	if houses[0][NATION_IDX] != NATION_NOR {
		return false
	}
	// The man who smokes Chesterfields lives in the house next to the man with the fox.
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_CHESTERFIELD {
			l := i - 1
			r := i + 1
			onLeft := l >= 0 && houses[l][PET_IDX] != PET_FOX
			onRight := r < 5 && houses[r][PET_IDX] != PET_FOX
			if !onLeft && !onRight {
				return false
			}
		}
	}
	// Kools are smoked in the house next to the house where the horse is kept
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_KOOLS {
			l := i - 1
			r := i + 1
			onLeft := l >= 0 && houses[l][PET_IDX] != PET_HORSE
			onRight := r < 5 && houses[r][PET_IDX] != PET_HORSE
			if !onLeft && !onRight {
				return false
			}
		}
	}
	// The Lucky Strike smoker drinks orange juice
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_LUCKYSTRIKE && houses[i][DRINK_IDX] != DRINK_ORANGE {
			return false
		}
	}
	// The Japanese smokes Parliaments
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_JP && houses[i][SMOKE_IDX] != SMOKE_PARLIAMENT {
			return false
		}
	}
	// The Norwegian lives next to the blue house
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_NOR {
			l := i - 1
			r := i + 1
			onLeft := l >= 0 && houses[l][COLOR_IDX] != COLOR_BLUE
			onRight := r < 5 && houses[r][COLOR_IDX] != COLOR_BLUE
			if !onLeft && !onRight {
				return false
			}
		}
	}
	return true
}
func valid(houses [5][5]int) bool {
	for j := 0; j < 5; j++ {
		vis := [5]bool{}
		for i := 0; i < 5; i++ {
			k := houses[i][j]
			if k >= 5 {
				continue
			}
			if vis[k] {
				return false
			}
			vis[k] = true
		}
	}
	return true
}

func findWaterDrinker(houses [5][5]int) int {
	for i := 0; i < 5; i++ {
		if houses[i][DRINK_IDX] == DRINK_WATER {
			return i
		}
	}
	return -1
}

func findZebraOwner(houses [5][5]int) int {
	for i := 0; i < 5; i++ {
		if houses[i][PET_IDX] == PET_ZEBRA {
			return i
		}
	}
	return -1
}

func SolvePuzzle() Solution {
	houses := [5][5]int{
		{5, 5, 5, 5, 5},
		{5, 5, 5, 5, 5},
		{5, 5, 5, 5, 5},
		{5, 5, 5, 5, 5},
		{5, 5, 5, 5, 5},
	}
	q := make([][3]int, 0)
	q = append(q, [3]int{0, 0, 0})
	for len(q) > 0 {
		i, j, v := q[0][0], q[0][1], q[0][2]
		q = q[:len(q)-1]
		houses[i][j] = v
		if valid(houses) {
			if i == 4 && j == 4 && compilant(houses) {
				waterDrinker := findWaterDrinker(houses)
				zebraOwner := findZebraOwner(houses)
				return Solution{
					DrinksWater: NATIONS[houses[waterDrinker][NATION_IDX]],
					OwnsZebra:   NATIONS[houses[zebraOwner][NATION_IDX]],
				}
			}
			if v < 4 {
				q = append(q, [3]int{i, j, v + 1})
			}
			j++
			if j == 5 {
				j = 0
				i++
			}
			if i == 5 {
				continue
			}
			q = append(q, [3]int{i, j, 0})
		} else {
			if v < 4 {
				q = append(q, [3]int{i, j, v + 1})
			} else {
				houses[i][j] = 5
			}
		}
	}
	fmt.Println("no solution!")
	return Solution{}
}
