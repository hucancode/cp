const std = @import("std");
const mem = std.mem;

fn count(garden: []const []const u8, i: usize, j: usize) u8 {
    var ret:u8 = 0;
    if (i > 0 and garden[i-1][j] == '*') ret += 1;
    if (i < garden.len-1 and garden[i+1][j] == '*') ret += 1;
    if (j > 0 and garden[i][j-1] == '*') ret += 1;
    if (j < garden[0].len-1 and garden[i][j+1] == '*') ret += 1;
    if (i > 0 and j > 0 and garden[i-1][j-1] == '*') ret += 1;
    if (i < garden.len-1 and j < garden[0].len-1 and garden[i+1][j+1] == '*') ret += 1;
    if (i > 0 and j < garden[0].len-1 and garden[i-1][j+1] == '*') ret += 1;
    if (i < garden.len-1 and j > 0 and garden[i+1][j-1] == '*') ret += 1;
    return if (ret > 0) ret + '0' else ' ';
}

pub fn annotate(allocator: mem.Allocator, garden: []const []const u8) mem.Allocator.Error![][]u8 {
    const ret = try allocator.alloc([]u8, garden.len);
    for(ret) |*row| row.* = &[_]u8{};
    errdefer {
        for(ret) |row| if (row.len > 0) allocator.free(row);
        allocator.free(ret);
    }
    for(ret) |*row| row.* = try allocator.alloc(u8, garden[0].len);
    for(0..ret.len) |i| {
        for(0..ret[i].len) |j| {
            switch (garden[i][j]) {
            ' ' => ret[i][j] = count(garden, i, j),
            else => ret[i][j] = garden[i][j],
            }
        }
    }
    return ret;
}
