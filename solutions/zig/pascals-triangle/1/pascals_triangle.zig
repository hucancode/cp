const std = @import("std");
const mem = std.mem;

pub fn rows(allocator: mem.Allocator, count: usize) mem.Allocator.Error![][]u128 {
    var ret = try allocator.alloc([]u128, count);
    if (count < 1) {
        return ret;
    }
    for(0..count) |i| {
        ret[i] = try allocator.alloc(u128, i+1);
        ret[i][0] = 1;
        ret[i][i] = 1;
        if (i <= 1) {
            continue;
        }
        for(1..i) |j| {
            ret[i][j] = ret[i-1][j] + ret[i-1][j-1];
        }
    }
    return ret;
}
