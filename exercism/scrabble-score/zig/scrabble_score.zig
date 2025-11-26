const mem = @import("std").mem;
const ascii = @import("std").ascii;
pub fn score(s: []const u8) u32 {
    const letters = [_][]const u8{ "AEIOULNRST", "DG", "BCMP", "FHVWY", "K", "JX", "QZ" };
    const scores = [_]u32{ 1, 2, 3, 4, 5, 8, 10 };
    var ret: u32 = 0;
    for (s) |c| {
        const needle = [_]u8{ascii.toUpper(c)};
        for (0..letters.len) |j| {
            const haystack = letters[j];
            if (mem.containsAtLeast(u8, haystack, 1, &needle)) {
                ret += scores[j];
                break;
            }
        }
    }
    return ret;
}
