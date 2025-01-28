const mem = @import("std").mem;
const ascii = @import("std").ascii;
const std = @import("std");

pub fn score(s: []const u8) u32 {
    const letters = [_][]const u8{ "AEIOULNRST", "DG", "BCMP", "FHVWY", "K", "JX", "QZ" };
    const scores = [_]u32{ 1, 2, 3, 4, 5, 8, 10 };
    var ret: u32 = 0;
    var scoreByLetter = std.AutoHashMap(u8, u32)
        .init(std.heap.page_allocator);
    for (0..letters.len) |i| {
        const x = scores[i];
        const haystack = letters[i];
        for (haystack) |c| {
            scoreByLetter.put(c, x) catch unreachable;
            scoreByLetter.put(ascii.toLower(c), x) catch unreachable;
        }
    }
    for (s) |c| {
        ret += scoreByLetter.get(c) orelse 0;
    }
    return ret;
}
