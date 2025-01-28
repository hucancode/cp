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

const UNKNOWN = 5

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

func maybeCompilant(houses [][]int) bool {
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_EN && (houses[i][COLOR_IDX] != COLOR_RED && houses[i][COLOR_IDX] != UNKNOWN) {
			fmt.Printf("house %d, english man but color %d\n", i, houses[i][COLOR_IDX])
			return false
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_SP && (houses[i][PET_IDX] != PET_DOG && houses[i][PET_IDX] != UNKNOWN) {
			fmt.Printf("house %d, spaniard but pet %d\n", i, houses[i][PET_IDX])
			return false
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][DRINK_IDX] == DRINK_COFFEE && (houses[i][COLOR_IDX] != COLOR_GREEN && houses[i][COLOR_IDX] != UNKNOWN) {
			fmt.Printf("house %d, coffee but color %d\n", i, houses[i][COLOR_IDX])
			return false
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_UKR && (houses[i][DRINK_IDX] != DRINK_TEA && houses[i][DRINK_IDX] != UNKNOWN) {
			fmt.Printf("house %d, ukrainian but drink %d\n", i, houses[i][DRINK_IDX])
			return false
		}
	}
	for i := 1; i < 5; i++ {
		if houses[i][COLOR_IDX] == COLOR_GREEN {
			if i == 0 || (houses[i-1][COLOR_IDX] != COLOR_IVORY && houses[i-1][COLOR_IDX] != UNKNOWN) {
				fmt.Printf("house %d, green but left house color %d\n", i, houses[i-1][COLOR_IDX])
				return false
			}
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_OLDGOLD && (houses[i][PET_IDX] != PET_SNAILS && houses[i][PET_IDX] != UNKNOWN) {
			fmt.Printf("house %d, smoke old gold but pet %d\n", i, houses[i][PET_IDX])
			return false
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_KOOLS && (houses[i][COLOR_IDX] != COLOR_YELLOW && houses[i][COLOR_IDX] != UNKNOWN) {
			fmt.Printf("house %d, smoke kools but color %d\n", i, houses[i][COLOR_IDX])
			return false
		}
	}
	if houses[2][DRINK_IDX] != DRINK_MILK && houses[2][DRINK_IDX] != UNKNOWN {
		fmt.Printf("house 2, drink %d not MILK\n", houses[2][DRINK_IDX])
		return false
	}
	if houses[0][NATION_IDX] != NATION_NOR && houses[0][NATION_IDX] != UNKNOWN {
		fmt.Printf("house 0, nation %d  not NOR\n", houses[0][NATION_IDX])
		return false
	}
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_CHESTERFIELD {
			l := i - 1
			r := i + 1
			onLeft := l < 0 || houses[l][PET_IDX] == PET_FOX || houses[l][PET_IDX] == UNKNOWN
			onRight := r >= 5 || houses[r][PET_IDX] == PET_FOX || houses[r][PET_IDX] == UNKNOWN
			if !onLeft && !onRight {
				fmt.Printf("house %d, smoke chesterfield but no fox around\n", i)
				return false
			}
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_KOOLS {
			l := i - 1
			r := i + 1
			onLeft := l < 0 || houses[l][PET_IDX] == PET_HORSE || houses[l][PET_IDX] == UNKNOWN
			onRight := r >= 5 || houses[r][PET_IDX] == PET_HORSE || houses[r][PET_IDX] == UNKNOWN
			if !onLeft && !onRight {
				fmt.Printf("house %d, smoke kools but no horse around\n", i)
				return false
			}
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][SMOKE_IDX] == SMOKE_LUCKYSTRIKE && (houses[i][DRINK_IDX] != DRINK_ORANGE && houses[i][DRINK_IDX] != UNKNOWN) {
			fmt.Printf("house %d, smoke lucky strike but drink %d\n", i, houses[i][DRINK_IDX])
			return false
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_JP && (houses[i][SMOKE_IDX] != SMOKE_PARLIAMENT && houses[i][SMOKE_IDX] != UNKNOWN) {
			fmt.Printf("house %d, japan but smoke %d\n", i, houses[i][SMOKE_IDX])
			return false
		}
	}
	for i := 0; i < 5; i++ {
		if houses[i][NATION_IDX] == NATION_NOR {
			l := i - 1
			r := i + 1
			onLeft := l < 0 || houses[l][COLOR_IDX] == COLOR_BLUE || houses[l][COLOR_IDX] == UNKNOWN
			onRight := r >= 5 || houses[r][COLOR_IDX] == COLOR_BLUE || houses[r][COLOR_IDX] == UNKNOWN
			if !onLeft && !onRight {
				fmt.Printf("house %d, norwegian but no blue around\n", i)
				return false
			}
		}
	}
	return true
}

func compilant(houses [][]int) bool {
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

func findWaterDrinker(houses [][]int) int {
	for i := 0; i < 5; i++ {
		if houses[i][DRINK_IDX] == DRINK_WATER {
			return i
		}
	}
	return -1
}

func findZebraOwner(houses [][]int) int {
	for i := 0; i < 5; i++ {
		if houses[i][PET_IDX] == PET_ZEBRA {
			return i
		}
	}
	return -1
}

func next(i, j int) (error, int, int) {
	j++
	if j >= UNKNOWN {
		j = 0
		i++
	}
	if i >= UNKNOWN {
		return fmt.Errorf("cant go next"), i, j
	}
	return nil, i, j
}

func SolvePuzzle() Solution {
	loop := 0
	houses := [][]int{
		{UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN},
		{UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN},
		{UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN},
		{UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN},
		{UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN},
	}
	taken := [5]uint8{
		0, 0, 0, 0, 0,
	}
	q := make([][3]int, 0)
	for v := 0; v < 5; v++ {
		q = append(q, [3]int{0, 0, v})
	}
	for len(q) > 0 {
		loop++
		if loop > 1000000 {
			fmt.Println("looping too much!")
			break
		}
		n := len(q) - 1
		i, j, v := q[n][0], q[n][1], q[n][2]
		q = q[:n]
		if v >= UNKNOWN {
			v -= UNKNOWN
			fmt.Printf("reset %d %d %d\n", i, j, v)
			taken[j] ^= 1 << v
			houses[i][j] = UNKNOWN
			continue
		}
		fmt.Printf("check %d %d %d\n", i, j, v)
		valid := taken[j]&(1<<v) == 0
		houses[i][j] = v
		if !valid {
			continue
		}
		taken[j] |= 1 << v
		q = append(q, [3]int{i, j, v + 5})
		if i == 4 && j == 4 {
			if !compilant(houses) {
				fmt.Printf("found a valid but not compilant houses %v\n", houses)
			} else {
				waterDrinker := findWaterDrinker(houses)
				zebraOwner := findZebraOwner(houses)
				return Solution{
					DrinksWater: NATIONS[houses[waterDrinker][NATION_IDX]],
					OwnsZebra:   NATIONS[houses[zebraOwner][NATION_IDX]],
				}
			}
		} else {
			if !maybeCompilant(houses) {
				fmt.Printf("found an invalid houses early %v\n", houses)
				continue
			}
		}
		fmt.Printf("push backtrack node %d, %d, %d\n", i, j, v)
		q = append(q, [3]int{i, j, v + UNKNOWN})
		err, i, j := next(i, j)
		if err != nil {
			fmt.Println("cant go next")
			continue
		}
		for v := 0; v < 5; v++ {
			fmt.Printf("next = %d %d %d\n", i, j, v)
			q = append(q, [3]int{i, j, v})
		}
	}
	fmt.Println("no solution!")
	return Solution{}
}
