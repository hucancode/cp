package resistor_color

Color :: enum {
	Black,
	Brown,
	Red,
	Orange,
	Yellow,
	Green,
	Blue,
	Violet,
	Grey,
	White,
}

code :: proc(color: Color) -> int {
	return int(color)
}

colors :: proc() -> [len(Color)]Color {
	ret: [len(Color)]Color
	for c, i in Color {
		ret[i] = c
	}
	return ret
}
