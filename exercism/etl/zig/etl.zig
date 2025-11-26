const std = @import("std");
const mem = std.mem;

pub fn transform(allocator: mem.Allocator, legacy: std.AutoHashMap(i5, []const u8)) mem.Allocator.Error!std.AutoHashMap(u8, i5) {
    var ret = std.AutoHashMap(u8, i5).init(allocator);
    var it = legacy.iterator();
    while (it.next()) |entry| {
        const score: i5 = entry.key_ptr.*;
        const letters: []const u8 = entry.value_ptr.*;
        for (letters) |c| try ret.put(std.ascii.toLower(c), score);
    }
    return ret;
}
