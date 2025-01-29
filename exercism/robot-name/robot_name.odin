package robotname

import "core:fmt"
import "core:math/rand"
import "core:strings"

names := make(map[string]bool)

Robot :: struct {
	name: string,
}

Error :: enum {
	None,
	CouldNotCreateName,
}

get_name :: proc(r: ^Robot) -> (string, Error) {
	if len(r.name) == 0 {
		r.name = create_name()
	}
	if len(r.name) == 0 {
		return r.name, Error.CouldNotCreateName
	}
	return r.name, Error.None
}

reset :: proc(r: ^Robot) {
	delete_key(&names, r.name)
	r.name = create_name()
}

letters := "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
numbers := "0123456789"
random_name :: proc() -> string {
	builder := strings.builder_make_len_cap(0, 5)
	for i in 0 ..< 2 {
		strings.write_byte(&builder, letters[rand.int_max(len(letters))])
	}
	for i in 0 ..< 3 {
		strings.write_byte(&builder, numbers[rand.int_max(len(numbers))])
	}
	return strings.to_string(builder)
}

create_name :: proc() -> string {
	max_tries := 1000
	for i in 0 ..< max_tries {
		key := random_name()
		if names[key] {
			delete(key)
			continue
		}
		names[key] = true
		return key
	}
	return ""
}
