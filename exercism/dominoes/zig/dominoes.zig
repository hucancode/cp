const std = @import("std");
const mem = std.mem;
const HashMap = std.AutoHashMap;
const ArrayList = std.ArrayList;

pub fn canChain(allocator: mem.Allocator, stones: []const [2]u3) !bool {
    if (stones.len < 1) return true;
    var head = HashMap(u3, ArrayList(usize)).init(allocator);
    var tail = HashMap(u3, ArrayList(usize)).init(allocator);
    defer {
        var it = head.valueIterator();
        while (it.next()) |e| e.*.deinit(allocator);
        head.deinit();
    }
    defer {
        var it = tail.valueIterator();
        while (it.next()) |e| e.*.deinit(allocator);
        tail.deinit();
    }
    // Fill head and tail maps
    for (stones, 0..) |stone, i| {
        const h_list = try head.getOrPut(stone[0]);
        if (!h_list.found_existing) {
            h_list.value_ptr.* = ArrayList(usize){};
        }
        try h_list.value_ptr.*.append(allocator, i);
        const t_list = try tail.getOrPut(stone[1]);
        if (!t_list.found_existing) {
            t_list.value_ptr.* = ArrayList(usize){};
        }
        try t_list.value_ptr.*.append(allocator, i);
    }
    var vis: []bool = try allocator.alloc(bool, stones.len);
    for (vis) |*v| v.* = false;
    defer allocator.free(vis);
    // Stack for DFS: (index, flipped)
    var stack = ArrayList(struct { idx: usize, flipped: bool }){};
    defer stack.deinit(allocator);
    try stack.append(allocator, .{ .idx = 0, .flipped = false });
    var path_len: usize = 0;
    while (stack.pop()) |node| {
        if (node.idx == stones.len) {
            if (path_len > 0) path_len -= 1;
            continue;
        }
        if (vis[node.idx]) continue;
        vis[node.idx] = true;
        path_len += 1;
        if (path_len == stones.len) {
            // Check if last connects to first
            const u = if (node.flipped) stones[node.idx][0] else stones[node.idx][1];
            const v = stones[0][0];
            if (u == v) return true;
            path_len -= 1;
            continue;
        }
        try stack.append(allocator, .{ .idx = stones.len, .flipped = false });
        const ab = stones[node.idx];
        const u = if (node.flipped) ab[0] else ab[1];
        if (head.get(u)) |list| {
            for (list.items) |i| try stack.append(allocator, .{ .idx = i, .flipped = false });
        }
        if (tail.get(u)) |list| {
            for (list.items) |i| try stack.append(allocator, .{ .idx = i, .flipped = true });
        }
    }
    return false;
}