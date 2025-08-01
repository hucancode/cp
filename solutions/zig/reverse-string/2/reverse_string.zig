const mem = @import("std").mem;
pub fn reverse(buffer: []u8, s: []const u8) []u8 {
    @memcpy(buffer, s);
    mem.reverse(u8, buffer);
    return buffer;
}
