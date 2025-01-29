package robotname

import "core:fmt"
import "core:math/rand"
import "core:strings"

RobotStorage :: struct {
	names: map[string]bool,
}

Robot :: struct {
	name: string,
}

Error :: enum {
	None,
	CouldNotCreateName,
}

new_storage :: proc() -> RobotStorage {
	return RobotStorage{make(map[string]bool)}
}
delete_storage :: proc(storage: ^RobotStorage) {
	delete_map(storage.names)
}

new_robot :: proc(storage: ^RobotStorage) -> (Robot, Error) {
	name, e := create_name(storage)
	return Robot{name}, e
}

reset :: proc(storage: ^RobotStorage, r: ^Robot) {
	delete_key(&storage.names, r.name)
	name, err := create_name(storage)
	if err != Error.None {
		return
	}
	delete_string(r.name)
	r.name = name
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

create_name :: proc(storage: ^RobotStorage) -> (string, Error) {
	max_tries := 100
	for i in 0 ..< max_tries {
		key := random_name()
		if key in storage.names {
			delete(key)
			continue
		}
		storage.names[key] = true
		return key, Error.None
	}
	return "", Error.CouldNotCreateName
}
