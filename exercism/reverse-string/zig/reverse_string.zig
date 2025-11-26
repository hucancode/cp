/// Writes a reversed copy of `s` to `buffer`.
pub fn reverse(buffer: []u8, s: []const u8) []u8 {
    const n = s.len;
    for(0..n) |i| {
        const j = n-1-i;
        buffer[j] = s[i];
    }
    return buffer[0..n];
}
