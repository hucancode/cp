package robotname

import "core:fmt"
import "core:strings"
import "core:testing"
import "core:text/regex"

seen := make(map[string]bool)

name_valid :: proc(name: string) -> bool {
	pat, err := regex.create(`^[A-Z]{2}\d{3}$`)
	if err != regex.Creation_Error.None {
		return false
	}
	_, matched := regex.match_and_allocate_capture(pat, name)
	return matched
}

@(test)
test_name_valid :: proc(t: ^testing.T) {
	name, e := get_name(&Robot{})
	testing.expect(t, e == Error.None)
	testing.expect(t, name_valid(name))
}

@(test)
test_name_sticks :: proc(t: ^testing.T) {
	r := Robot{}
	n1, e1 := get_name(&r)
	n2, e2 := get_name(&r)
	testing.expect(t, e1 == Error.None)
	testing.expect(t, e2 == Error.None)
	testing.expect_value(t, n1, n2)
}

@(test)
test_successive_robots_have_different_names :: proc(t: ^testing.T) {
	n1, e1 := get_name(&Robot{})
	n2, e2 := get_name(&Robot{})
	testing.expect(t, e1 == Error.None)
	testing.expect(t, e2 == Error.None)
	testing.expect(t, n1 != n2)
}

@(test)
test_reset_name :: proc(t: ^testing.T) {
	r := Robot{}
	n1, e1 := get_name(&r)
	reset(&r)
	n2, e2 := get_name(&r)
	testing.expect(t, e1 == Error.None)
	testing.expect(t, e2 == Error.None)
	testing.expect(t, n1 != n2)
}

@(test)
test_multiple_names :: proc(t: ^testing.T) {
	for i := len(seen); i <= 1000; i += 1 {
		name, err := get_name(&Robot{})
		testing.expect(t, err == Error.None)
		testing.expect(t, !seen[name])
		seen[name] = true
	}
}


@(test)
test_collisions :: proc(t: ^testing.T) {
	if len(seen) < 1 << 32 {
		return
	}
	max_names :: 26 * 26 * 10 * 10 * 10
	lots_of_names :: 76000
	for i := len(seen); i <= lots_of_names; i += 1 {
		get_name(&Robot{})
	}
	r := Robot{}
	for i := len(seen); i < max_names; i += 1 {
		reset(&r)
		get_name(&r)
	}
	_, err := get_name(&Robot{})
	testing.expect(t, err == Error.CouldNotCreateName)
}
