const mem = @import("std").mem;
pub fn reverse(buffer: []u8, s: []const u8) []u8 {
    const n = s.len;
    @memcpy(buffer[0..n], s);
    mem.reverse(u8, buffer[0..n]);
    return buffer[0..n];
}
