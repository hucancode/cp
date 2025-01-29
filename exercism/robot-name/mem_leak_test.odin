package main
import "core:fmt"
import "core:math/rand"
import "core:strings"
import "core:testing"

letters := "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

random_name :: proc() -> string {
	n := 5
	ret := make([]u8, n)
	for i in 0 ..< n {
		ret[i] = letters[rand.int_max(len(letters))]
	}
	return string(ret)
}

create_name :: proc(seen: ^map[string]bool) -> string {
	n := 10
	for i in 0 ..< n {
		name := random_name()
		if seen[name] {
			delete(name)
			continue
		}
		seen[name] = true
		return name
	}
	return ""
}

//@(test)
test_generate_2_names :: proc(t: ^testing.T) {
	seen := make(map[string]bool)
	defer delete(seen)
	defer for k in seen {
		delete(k)
	}
	n1 := create_name(&seen)
	n2 := create_name(&seen)
	testing.expect(t, n1 != n2)
}

dfs_fill_names :: proc(names: ^map[string]bool) {
	GO_BACK_SENTINEL :: u8('-')
	NAME_LENGTH :: 5
	LETTERS := "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	NUMBERS := "0123456789"
	cap := len(LETTERS) * NAME_LENGTH
	stack := make_dynamic_array_len_cap([dynamic]u8, 0, cap)
	defer delete(stack)
	current := make([]u8, NAME_LENGTH)
	defer delete(current)
	for i in 0 ..< len(LETTERS) {
		append(&stack, LETTERS[i])
	}
	depth := 0
	for len(stack) > 0 {
		ch := pop(&stack)
		go_back := ch == GO_BACK_SENTINEL
		if go_back {
			depth -= 1
			continue
		}
		current[depth] = ch
		depth += 1
		if depth == NAME_LENGTH {
			key := string(current)
			names[strings.clone(key)] = true
			n := len(names)
			if n % 8673 == 0 {
				fmt.printfln("[%d] '%s',", n, key)
			}
			depth -= 1
			continue
		}
		append(&stack, GO_BACK_SENTINEL)
		if depth < 2 {
			for i in 0 ..< len(LETTERS) {
				append(&stack, LETTERS[i])
			}
		} else {
			for i in 0 ..< len(NUMBERS) {
				append(&stack, NUMBERS[i])
			}
		}
	}
}

@(test)
test_populate :: proc(t: ^testing.T) {
	names := make(map[string]bool)
	defer delete(names)
	defer for k in names {
		delete(k)
	}
	dfs_fill_names(&names)
}
