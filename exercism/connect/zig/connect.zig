const std = @import("std");
const mem = std.mem;

const Pos = struct { i: usize, j: usize };
const Delta = struct { di: isize, dj: isize };

fn bfs(
    allocator: mem.Allocator,
    mat: []const []const u8,
    si: usize,
    sj: usize,
    targeti: ?usize,
    targetj: ?usize,
) !bool {
    const c = mat[si][sj];
    if (c == '.') return false;
    const n = mat.len;
    var vis = try allocator.alloc([]bool, n);
    defer {
        for (vis) |row| allocator.free(row);
        allocator.free(vis);
    }
    for (vis, 0..) |*row_ptr, r| {
        row_ptr.* = try allocator.alloc(bool, mat[r].len);
        @memset(row_ptr.*, false);
    }
    var q = std.ArrayList(Pos){};
    defer q.deinit(allocator);
    var head: usize = 0;
    try q.append(allocator, .{ .i = si, .j = sj });
    const moves = [_]Delta{
        .{ .di = -1, .dj = 0 },
        .{ .di = 1,  .dj = 0 },
        .{ .di = 0,  .dj = -1 },
        .{ .di = 0,  .dj = 1 },
        .{ .di = -1, .dj = 1 },
        .{ .di = 1,  .dj = -1 },
    };

    while (head < q.items.len) {
        const p = q.items[head];
        head += 1;
        if (vis[p.i][p.j]) continue;
        vis[p.i][p.j] = true;
        if ((targeti == null or targeti.? == p.i) and
            (targetj == null or targetj.? == p.j))
        {
            return true;
        }
        for (moves) |d| {
            const ni_s = @as(isize, @intCast(p.i)) + d.di;
            const nj_s = @as(isize, @intCast(p.j)) + d.dj;
            if (ni_s < 0 or nj_s < 0) continue;
            const ni = @as(usize, @intCast(ni_s));
            const nj = @as(usize, @intCast(nj_s));
            if (ni >= n or nj >= mat[ni].len) continue;
            if (mat[ni][nj] == c) {
                try q.append(allocator, .{ .i = ni, .j = nj });
            }
        }
    }
    return false;
}

pub fn winner(allocator: mem.Allocator, board: []const []const u8) !u8 {
    const n = board.len;
    if (n == 0) return '.';
    // Normalize board: remove spaces from each row
    var norm = try allocator.alloc([]const u8, n);
    defer {
        for (norm) |row| allocator.free(row);
        allocator.free(norm);
    }
    for (board, 0..) |row, i| {
        var buf = std.ArrayList(u8){};
        defer buf.deinit(allocator);
        for (row) |ch| if (ch != ' ') try buf.append(allocator, ch);
        norm[i] = try buf.toOwnedSlice(allocator);
    }
    const m = norm[0].len;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        if (try bfs(allocator, norm, i, 0, null, m - 1)) {
            return norm[i][0];
        }
    }
    var j: usize = 0;
    while (j < m) : (j += 1) {
        if (try bfs(allocator, norm, 0, j, n - 1, null)) {
            return norm[0][j];
        }
    }
    return '.';
}
