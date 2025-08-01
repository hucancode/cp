const std = @import("std");
const mem = std.mem;

pub fn detectAnagrams(
    allocator: mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var ret = std.BufSet.init(allocator);
    const n = word.len;
    var wordNormalized = try allocator.alloc(u8, n);
    defer allocator.free(wordNormalized);
    for (0..n) |i| {
        wordNormalized[i] = std.ascii.toLower(word[i]) - 'a';
    }
    var wordSignature = try allocator.alloc(u8, 26);
    defer allocator.free(wordSignature);
    for (0..n) |i| {
        wordSignature[wordNormalized[i]] += 1;
    }
    for (candidates) |candidate| {
        if (candidate.len != n) {
            continue;
        }
        var candidateNormalized = try allocator.alloc(u8, n);
        defer allocator.free(candidateNormalized);
        for (0..n) |i| {
            candidateNormalized[i] = std.ascii.toLower(candidate[i]) - 'a';
        }
        if (mem.eql(u8, wordNormalized, candidateNormalized)) {
            continue;
        }
        var candidateSignature = try allocator.alloc(u8, 26);
        defer allocator.free(candidateSignature);
        for (0..n) |i| {
            candidateSignature[candidateNormalized[i]] += 1;
        }
        if (mem.eql(u8, wordSignature, candidateSignature)) {
            try ret.insert(candidate);
        }
    }
    return ret;
}
