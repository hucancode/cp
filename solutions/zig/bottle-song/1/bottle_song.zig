const std = @import("std");
const bufPrint = std.fmt.bufPrint;

const nums = [_][]const u8{"no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"};
const cnums =[_][]const u8{"no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"};
pub fn recite(buffer: []u8, start_bottles: u32, take_down: u32) []const u8 {
    var n: usize = 0;
    for(0..take_down) |i| {
        const j = start_bottles - i;
        const k = j - 1;
        const a = cnums[j];
        const b = if(j != 1) "s" else "";
        const c = nums[k];
        const d = if(k != 1) "s" else "";
        n += (bufPrint(buffer[n..], "{s} green bottle{s} hanging on the wall,\n{s} green bottle{s} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {s} green bottle{s} hanging on the wall.\n\n", .{a, b, a, b, c, d}) catch buffer[0..0]).len;
    }
    return std.mem.trimRight(u8, buffer[0..n], "\n");
}
