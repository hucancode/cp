const std = @import("std");
const mem = std.mem;

pub const ChangeError = error{
    NegativeTarget,
    UnreachableTarget,
};

pub fn findFewestCoins(
    allocator: mem.Allocator,
    coins: []const u64,
    target: i64,
) (mem.Allocator.Error || ChangeError)![]u64 {
    if (target < 0) return ChangeError.NegativeTarget;
    if (target == 0) return &[_]u64{};
    const limit: usize = @intCast(target);
    const INF: u64 = @intCast(target + 1);
    // f[x] = minimum coins to reach x
    const f = try allocator.alloc(u64, limit + 1);
    defer allocator.free(f);
    const Prev = struct { prev_sum: usize, coin: u64 };
    const prev = try allocator.alloc(?Prev, limit + 1);
    defer allocator.free(prev);
    @memset(f, INF);
    @memset(prev, null);
    f[0] = 0;
    for (0..limit + 1) |i| {
        if (f[i] == INF) continue;
        const base = f[i];
        for (coins) |c| {
            const ni = i + @as(usize, c);
            if (ni > limit) continue;
            const cand = base + 1;
            if (cand < f[ni]) {
                f[ni] = cand;
                prev[ni] = Prev{ .prev_sum = i, .coin = c };
            }
        }
    }
    if (f[limit] == INF) return ChangeError.UnreachableTarget;
    // reconstruct
    var result = std.ArrayList(u64){};
    errdefer result.deinit(allocator);
    var cur = limit;
    while (cur != 0) {
        const p = prev[cur] orelse return ChangeError.UnreachableTarget;
        try result.append(allocator, p.coin);
        cur = p.prev_sum;
    }
    mem.sort(u64, result.items, {}, comptime std.sort.asc(u64));
    return result.toOwnedSlice(allocator);
}
