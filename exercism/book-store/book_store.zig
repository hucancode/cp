const std = @import("std");
fn calculate(counts: []const u32) u32 {
    var ret: f32 = 0.0;
    var used: u32 = 0;
    const base_price = 800.0;
    const n = counts.len;
    for (0..n) |i| {
        const m = n - i;
        const x = counts[i] - used;
        used = counts[i];
        const price = base_price * @as(f32, @floatFromInt(m)) * @as(f32, @floatFromInt(x));
        const discount: f32 = switch (m) {
            2 => 0.95,
            3 => 0.90,
            4 => 0.80,
            5 => 0.75,
            else => 1.00,
        };
        ret += price * discount;
    }
    return @as(u32, @intFromFloat(ret));
}

pub fn total(basket: []const u32) u32 {
    var counts = std.mem.zeroes([5]u32);
    for (basket) |book| {
        counts[book - 1] += 1;
    }
    std.mem.sort(u32, &counts, {}, comptime std.sort.asc(u32));
    const delta = @min(counts[0], counts[2] - counts[1]);
    counts[0] -= delta;
    counts[1] += delta;
    return calculate(&counts);
}
