const std = @import("std");
const mem = std.mem;
const ArrayList = std.ArrayList;

pub fn factors(allocator: mem.Allocator, value: u64) mem.Allocator.Error![]u64 {
    var ret = ArrayList(u64).init(allocator);
    var i: u64 = 2;
    var n = value;
    const sn = @as(u64, @intFromFloat(@sqrt(@as(f64, @floatFromInt(value)))));
    while(i <= n and i <= sn) {
        while(n % i == 0) {
            try ret.append(i);
            n /= i;
        }
        i += 1;
    }
    if (n > 1) {
        try ret.append(n);
    }
    return ret.toOwnedSlice();
}
