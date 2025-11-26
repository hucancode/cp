const std = @import("std");
const mem = std.mem;

pub const Item = struct {
    weight: usize,
    value: usize,
    pub fn init(weight: usize, value: usize) Item {
        return Item {.weight = weight, .value = value };
    }
};

pub fn maximumValue(allocator: mem.Allocator, maximumWeight: usize, items: []const Item) !usize {
    const n = maximumWeight+1;
    var f = try allocator.alloc(usize, n);
    @memset(f[0..n], 0);
    defer allocator.free(f);
    for (items) |item| {
        var i = maximumWeight;
        while (i >= item.weight) : (i -= 1) {
            f[i] = @max(f[i], f[i-item.weight]+item.value);
        }
    }
    return f[maximumWeight];
}
