const std = @import("std");
const vowels = "aeiou";

fn indexOfVowel(word: []const u8) ?usize {
    return std.mem.indexOfAny(u8, word, vowels);
}

fn indexOfNoVowel(haysack: []const u8, needle: []const u8) ?usize {
    const i = std.mem.indexOfAny(u8, haysack, vowels);
    const j = std.mem.indexOf(u8, haysack, needle);
    if (i == null or (j != null and j.? < i.?)) {
        return j;
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
