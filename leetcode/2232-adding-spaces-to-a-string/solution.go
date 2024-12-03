func addSpaces(s string, spaces []int) string {
    var builder strings.Builder
    builder.Grow(len(s) + len(spaces))
    i := 0
	for _,j := range spaces {
		builder.WriteString(s[i:j])
		builder.WriteByte(' ')
        i = j
	}
    builder.WriteString(s[i:])
    return builder.String()
}
