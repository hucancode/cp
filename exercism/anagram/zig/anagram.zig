const std = @import("std");
const toLower = std.ascii.toLower;
const mem = std.mem;

pub fn detectAnagrams(
    allocator: mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var ret = std.BufSet.init(allocator);
    errdefer ret.deinit();
    const n = word.len;
    var wordHash = try allocator.alloc(u8, 26);
    defer allocator.free(wordHash);
    @memset(wordHash, 0);
    var candidateHash = try allocator.alloc(u8, 26);
    defer allocator.free(candidateHash);
    for (word) |c| wordHash[toLower(c) - 'a'] += 1;
    for (candidates) |candidate| {
        if (candidate.len != n) continue;
        @memset(candidateHash, 0);
        var diff = false;
        for (word, candidate) |w,c| {
            const a = toLower(w);
            const b = toLower(c);
            diff |= a != b;
            candidateHash[b - 'a'] += 1;
        }
        if (diff and mem.eql(u8, wordHash, candidateHash)) try ret.insert(candidate);
    }
    return ret;
}
