const std = @import("std");

fn isVowel(c: u8) bool {
    return c == 'a' or c == 'i' or c == 'u' or c == 'e' or c == 'o';
}

fn indexOfVowel(word: []const u8) ?usize {
    const n = word.len;
    for (0..n) |i| {
        if (isVowel(word[i])) {
            return i;
        }
    }
    return null;
}

fn indexOfNoVowel(haysack: []const u8, needle: []const u8) ?usize {
    const n = haysack.len;
    for (0..n) |i| {
        if (isVowel(haysack[i])) {
            return null;
        }
        if (std.mem.startsWith(u8, haysack[i..], needle)) {
            return i;
        }
    }
    return null;
}

pub fn translate(allocator: std.mem.Allocator, phrase: []const u8) std.mem.Allocator.Error![]u8 {
    var it = std.mem.splitScalar(u8, phrase, ' ');
    var ret = std.ArrayList(u8).init(allocator);
    while (it.next()) |word| {
        const beginWithXrYt = std.mem.startsWith(u8, word, "xr") or std.mem.startsWith(u8, word, "yt");
        const quIdx = indexOfNoVowel(word, "qu");
        const yIdx = indexOfNoVowel(word, "y");
        const vowelIdx = indexOfVowel(word);
        var n: usize = 0;
        if (quIdx != null) {
            n = quIdx.? + 2;
        } else if (yIdx != null and yIdx.? > 0) {
            n = yIdx.?;
        } else if (!beginWithXrYt and vowelIdx != null) {
            n = vowelIdx.?;
        }
        try ret.appendSlice(word[n..]);
        try ret.appendSlice(word[0..n]);
        try ret.appendSlice("ay ");
    }
    _ = ret.pop();
    return try ret.toOwnedSlice();
}
