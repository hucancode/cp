const std = @import("std");
const mem = std.mem;
const ascii = std.ascii;
const ArrayList = std.ArrayList;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var arr = ArrayList(u8).init(allocator);
    defer arr.deinit();
    var it = mem.splitAny(u8, words, ",- ");
    while (it.next()) |content| {
        const w = mem.trim(u8, content, "_");
        if(w.len > 0) {
            try arr.append(ascii.toUpper(w[0]));
        }
    }
    return arr.toOwnedSlice();
}
