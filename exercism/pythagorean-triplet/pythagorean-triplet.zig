const std = @import("std");
const mem = std.mem;

pub const Triplet = struct {
    a: usize,
    b: usize,
    c: usize,
    pub fn init(a: usize, b: usize, c: usize) Triplet {
        return Triplet{ .a = a, .b = b, .c = c };
    }
};

pub fn tripletsWithSum(allocator: mem.Allocator, n: usize) mem.Allocator.Error![]Triplet {
    const na = n / 3 + 1;
    var list = std.ArrayList(Triplet).init(allocator);
    defer list.deinit();
    for (1..na) |a| {
        const nb = (n - a) / 2 + 1;
        for (a + 1..nb) |b| {
            const c = n - a - b;
            if (c < b or c > a + b or b > a + c or a > b + c) continue;
            if (a * a + b * b != c * c) {
                continue;
            }
            try list.append(Triplet.init(a, b, c));
        }
    }
    return list.toOwnedSlice();
}
