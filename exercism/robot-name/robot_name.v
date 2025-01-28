module main

import rand

struct Robot {
  mut:
    name string
}

struct RobotStorage {
  mut:
    names map[string]bool
}

fn random_name() string {
  letters := 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
  numbers := '0123456789'
  ab := rand.string_from_set(letters, 2)
  iii := rand.string_from_set(numbers, 3)
  return ab + iii
}

fn (r RobotStorage) contains(needle string) bool {
  return r.names[needle]
}

fn create_robot_storage() RobotStorage {
  return RobotStorage { names: map[string]bool{} }
}

fn (mut r RobotStorage) create_name() string {
  max_tries := 1000
  for i := 0; i < max_tries; i++ {
    name := random_name()
    if !r.contains(name) {
      r.names[name] = true
      return name
    }
  }
  return ""
}

fn create_robot(mut robots RobotStorage) Robot {
  return Robot { name: robots.create_name() }
}

fn (mut r Robot) reset(mut robots RobotStorage) {
  robots.names.delete(r.name)
  r.name = robots.create_name()
}
