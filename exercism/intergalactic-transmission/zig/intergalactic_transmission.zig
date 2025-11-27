const std = @import("std");
const mem = std.mem;

pub const TransmissionError = error{
    WrongParity,
};

pub fn transmitSequence(allocator: mem.Allocator, message: []const u8) mem.Allocator.Error![]u8 {
    if (message.len == 0) {
        return try allocator.alloc(u8, 0);
    }
    const n = message.len + message.len/8 + 1;
    var ret = try allocator.alloc(u8, n);
    var bptr: usize = 0;
    var k: usize = 0;
    while (bptr < message.len*8) : (bptr += 7) {
        const i = bptr/8;
        const j = (bptr+6)/8;
        const hi = message[i];
        const lo = if(j < message.len) message[j] else 0;
        const bhi = @as(u3, @intCast(bptr%8));
        const blo = @as(u3, @intCast(@min(7, 8-bptr%8)));
        const maskHi = @as(u8, 0xfe) >> bhi;
        const maskLo = @as(u8, 0xfe) << blo;
        const x = ((hi & maskHi) << bhi) | ((lo & maskLo) >> blo);
        const parity = @popCount(x)%2;
        ret[k] = x | parity;
        k += 1;
    }
    return ret;
}

pub fn decodeMessage(allocator: mem.Allocator, message: []const u8) (mem.Allocator.Error || TransmissionError)![]u8 {
    if (message.len == 0) {
        return try allocator.alloc(u8, 0);
    }
    const n = (message.len*8 - message.len)/8;
    var ret = try allocator.alloc(u8, n);
    @memset(ret, 0);
    var bptr: usize = 0;
    var k: usize = 0;
    while (bptr < ret.len*8) : (bptr += 7) {
        var x = message[k];
        k += 1;
        if (@popCount(x)%2 != 0) {
            allocator.free(ret);
            return TransmissionError.WrongParity;
        }
        x &= 0xfe;
        const i = bptr/8;
        const j = (bptr+7)/8;
        const bhi = @as(u3, @intCast(bptr%8));
        const blo = @as(u3, @intCast(@min(7, 8-bptr%8)));
        const hi = x >> bhi;
        const lo = x << blo;
        ret[i] |= hi;
        if (j < ret.len) ret[j] |= lo;
    }
    return ret;
}
