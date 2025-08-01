const std = @import("std");
const mem = std.mem;
const ArrayList = std.ArrayList;

pub fn factors(allocator: mem.Allocator, value: u64) mem.Allocator.Error![]u64 {
    var ret = ArrayList(u64).init(allocator);
    var i: u64 = 2;
    var k = value;
    const n = @as(u64, @intFromFloat(@sqrt(@as(f64, @floatFromInt(value)))));
    while(i <= k and i <= n) {
        if(k % i != 0) {
            i += 1;
            continue;
        }
        try ret.append(i);
        k /= i;
    }
    if (k > 1) {
        try ret.append(k);
    }
    return ret.toOwnedSlice();
}
