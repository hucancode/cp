package robotname

import (
	"fmt"
	"math/rand"
	"strings"
)

var names = map[string]bool{}

type Robot struct {
	name string
}

func (r *Robot) Name() (string, error) {
	if len(r.name) == 0 {
		r.name = createName()
	}
	if len(r.name) == 0 {
		return r.name, fmt.Errorf("robot name is empty")
	}
	return r.name, nil
}

func (r *Robot) Reset() {
	delete(names, r.name)
	r.name = createName()
}

func randomName() string {
	letters := "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	numbers := "0123456789"
	ab := randomStringFromSet(letters, 2)
	iii := randomStringFromSet(numbers, 3)
	return ab + iii
}

func randomStringFromSet(set string, length int) string {
	var sb strings.Builder
	for i := 0; i < length; i++ {
		sb.WriteByte(set[rand.Intn(len(set))])
	}
	return sb.String()
}

// createName generates a unique name for a robot, ensuring no duplicates.
func createName() string {
	maxTries := 1000
	for i := 0; i < maxTries; i++ {
		name := randomName()
		if _, ok := names[name]; ok {
			continue
		}
		names[name] = true
		return name
	}
	return ""
}
