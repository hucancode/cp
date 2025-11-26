const std = @import("std");
const mem = std.mem;

pub const Item = struct {
    // This struct, as well as its fields and init method, needs to be implemented.
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
        var g = try allocator.alloc(usize, n);
        g[0] = 0;
        for (1..n) |i| {
            g[i] = @max(f[i], g[i-1]);
            if (item.weight <= i) {
                g[i] = @max(g[i], f[i-item.weight]+item.value);
            }
        }
        allocator.free(f);
        f = g;
    }
    return f[maximumWeight];
}
