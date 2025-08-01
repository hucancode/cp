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
	ab := randomPick(2, letters)
	iii := randomPick(3, numbers)
	return ab + iii
}

func randomPick(length int, pool string) string {
	var sb strings.Builder
	n := len(pool)
	for i := 0; i < length; i++ {
		sb.WriteByte(pool[rand.Intn(n)])
	}
	return sb.String()
}

func createName() string {
	maxTries := 1000
	for i := 0; i < maxTries; i++ {
		key := randomName()
		if _, ok := names[key]; ok {
			continue
		}
		names[key] = true
		return key
	}
	return ""
}
