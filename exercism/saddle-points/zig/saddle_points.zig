const std = @import("std");
const mem = std.mem;

pub const Point = struct {
    row: u16,
    column: u16,
};

pub fn saddlePoints(comptime m: usize, comptime n: usize, allocator: mem.Allocator, matrix: [m][n]i32) mem.Allocator.Error![]Point {
    const bestInRow = try allocator.alloc(i32, m);
    defer allocator.free(bestInRow);
    @memset(bestInRow, 0);
    const bestInCol = try allocator.alloc(i32, n);
    @memset(bestInCol, std.math.maxInt(i32));
    defer allocator.free(bestInCol);
    for (0..m) |i| {
        for (0..n) |j| {
            bestInRow[i] = @max(bestInRow[i], matrix[i][j]);
            bestInCol[j] = @min(bestInCol[j], matrix[i][j]);
        }
    }
    var ret = std.ArrayList(Point){};
    defer ret.deinit(allocator);
    for (0..m) |i| {
        for (0..n) |j| {
            if (bestInRow[i] == matrix[i][j] and bestInCol[j] == matrix[i][j]) {
                try ret.append(allocator, .{ .row = @intCast(i+1), .column = @intCast(j+1) });
            }
        }
    }
    return try ret.toOwnedSlice(allocator);
}
