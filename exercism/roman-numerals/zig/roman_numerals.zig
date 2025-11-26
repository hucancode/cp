const std = @import("std");
const mem = std.mem;
const ArrayList = std.ArrayList;

pub fn toRoman(allocator: mem.Allocator, arabicNumeral: i16) mem.Allocator.Error![]u8 {
    const arabic = [_]i16 {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
    const roman = [_][]const u8 {"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"};
    var n = arabicNumeral;
    var ret = ArrayList(u8).init(allocator);
    defer ret.deinit();
    for(0..arabic.len) |i| {
        while(n >= arabic[i]) {
            try ret.appendSlice(roman[i]);
            n -= arabic[i];
        }
    }
    return ret.toOwnedSlice();
}
