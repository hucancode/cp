const mem = @import("std").mem;

pub const Signal = enum {
    wink,
    double_blink,
    close_your_eyes,
    jump,
};

pub fn calculateHandshake(allocator: mem.Allocator, number: u5) mem.Allocator.Error![]const Signal {
    const n: usize = @popCount(number & 0b1111);
    var i: usize = 0;
    var ret = try allocator.alloc(Signal, n);
    for (0..4) |b| {
        const mask = @as(u5, 1) << @as(u3, @intCast(b));
        if (number & mask != 0) {
            ret[i] = @enumFromInt(b);
            i += 1;
        }
    }
    if ((number & 1<<4) != 0) {
        for (0..n/2) |j| {
            const tmp = ret[j];
            ret[j] = ret[n-j-1];
            ret[n-j-1] = tmp;
        }
    }
    return ret[0..n];
}
