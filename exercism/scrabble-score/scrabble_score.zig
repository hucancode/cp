const mem = @import("std").mem;
const ascii = @import("std").ascii;

pub fn score(s: []const u8) u32 {
    const letters = [_][]const u8{ "AEIOULNRST", "DG", "BCMP", "FHVWY", "K", "JX", "QZ" };
    const scores = [_]u32{ 1, 2, 3, 4, 5, 8, 10 };
    var ret: u32 = 0;
    var scoreByLetter = mem.zeroes([26]u32);
    for (0..letters.len) |i| {
        for (letters[i]) |c| {
            scoreByLetter[c - 'A'] = scores[i];
        }
    }
    for (s) |c| {
        ret += scoreByLetter[ascii.toUpper(c) - 'A'];
    }
    return ret;
}
