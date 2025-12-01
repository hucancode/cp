const std = @import("std");
const mem = std.mem;
const ArrayList = std.ArrayList;

pub const Box = union(enum) {
    none,
    one: i12,
    many: []const Box,
};

pub fn flatten(allocator: mem.Allocator, box: Box) mem.Allocator.Error![]i12 {
    var ret = ArrayList(i12){};
    defer ret.deinit(allocator);
    var stack = ArrayList(Box){};
    defer stack.deinit(allocator);
    try stack.append(allocator, box);
    while (stack.pop()) |b| {
        switch (b) {
            .one => |*x| try ret.append(allocator, x.*),
            .many => |a| for (0..a.len) |i| try stack.append(allocator, a[a.len - 1 - i]),
            .none => continue,
        }
        
    }
    return try ret.toOwnedSlice(allocator);
}
