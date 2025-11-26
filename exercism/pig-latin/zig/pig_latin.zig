const mem = @import("std").mem;const debug = @import("std").debug;
const ArrayList = @import("std").ArrayList;

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

pub fn translate(allocator: mem.Allocator, phrase: []const u8) mem.Allocator.Error![]u8 {
    var it = mem.splitScalar(u8, phrase, ' ');
    var ret = ArrayList(u8).init(allocator);
    while (it.next()) |word| {
        const beginWithVowel = word.len >= 1 and isVowel(word[0]);
        const beginWithXrYt = word.len >= 2 and (
            mem.eql(u8, word[0..2], "xr") or 
            mem.eql(u8, word[0..2], "yt")
        );
        const quIndex = mem.indexOf(u8, word, "qu");
        const yIndex = mem.indexOf(u8, word, "y");
        const noVowelBeforeQu = quIndex != null and (yIndex == null or yIndex.? > quIndex.?) and indexOfVowel(word[0..quIndex.?]) == null;
        const noVowelBeforeY = yIndex != null and yIndex.? > 0 and (quIndex == null or quIndex.? > yIndex.?) and indexOfVowel(word[0..yIndex.?]) == null;
        const vowelIndex = indexOfVowel(word);
        
        if (noVowelBeforeQu) {
            const n = quIndex.?+2;
            try ret.appendSlice(word[n..]);
            try ret.appendSlice(word[0..n]);
            try ret.appendSlice("ay");
        } else if (noVowelBeforeY) {
            const n = yIndex.?;
            try ret.appendSlice(word[n..]);
            try ret.appendSlice(word[0..n]);
            try ret.appendSlice("ay");
        } else if (beginWithVowel or beginWithXrYt) {
            try ret.appendSlice(word);
            try ret.appendSlice("ay");
        } else if (vowelIndex != null) {
            const n = vowelIndex.?;
            try ret.appendSlice(word[n..]);
            try ret.appendSlice(word[0..n]);  
            try ret.appendSlice("ay");
        }
        try ret.append(' ');
    }
    _ = ret.pop();
    return try ret.toOwnedSlice();
}
