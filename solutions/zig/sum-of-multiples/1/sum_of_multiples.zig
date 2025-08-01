const std = @import("std");
const mem = std.mem;

pub fn sum(allocator: mem.Allocator, factors: []const u32, limit: u32) !u64 {
    var map = std.AutoHashMap(u32, bool).init(allocator);
    defer map.deinit();
    for(factors) |x| {
        if(x == 0) {
            continue;
        }
        var i = x;
        while(i < limit) {
            try map.put(i, true);
            i += x;
        }
    }
    var it = map.keyIterator();
    var ret: u64 = 0;
    while (it.next()) |k| {
        ret += k.*;
    }
    return ret;
}
