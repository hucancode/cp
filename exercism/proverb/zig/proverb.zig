const std = @import("std");
const fmt = std.fmt;
const bufPrint = fmt.bufPrint;
const mem = std.mem;

pub fn recite(allocator: mem.Allocator, words: []const []const u8) (fmt.AllocPrintError || mem.Allocator.Error)![][]u8 {
    var ret = try allocator.alloc([]u8, words.len);
    if (words.len == 0) {
        return ret;
    }
    for(0..words.len-1) |i| {
        const n = "For want of a  the  was lost.\n".len + words[i].len + words[i+1].len;
        ret[i] = try allocator.alloc(u8, n);
        _ = bufPrint(ret[i], "For want of a {s} the {s} was lost.\n", .{words[i], words[i+1]}) catch return error.OutOfMemory;
    }
    const n = "And all for the want of a .\n".len + words[0].len;
    ret[words.len-1] = try allocator.alloc(u8, n);
    _ = bufPrint(ret[words.len-1], "And all for the want of a {s}.\n", .{words[0]}) catch return error.OutOfMemory;
    return ret;
}
