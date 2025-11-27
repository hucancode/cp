const std = @import("std");
const mem = std.mem;

pub const TransmissionError = error{
    WrongParity,
};

pub fn transmitSequence(allocator: mem.Allocator, message: []const u8) mem.Allocator.Error![]u8 {
    if (message.len == 0) {
        return try allocator.alloc(u8, 0);
    }
    const n = message.len + message.len / 8 + 1;
    var ret = try allocator.alloc(u8, n);
    var bit: usize = 0;
    while (bit < message.len * 8) : (bit += 7) {
        const i = bit / 8;
        const j = i + 1;
        const hi = message[i];
        const lo = if (j < message.len) message[j] else 0;
        const shiftHi: u3 = @intCast(bit & 7);
        const shiftLo: u3 = @intCast(@min(7, 8 - (bit & 7)));
        const maskHi = @as(u8, 0xfe) >> shiftHi;
        const maskLo = @as(u8, 0xfe) << shiftLo;
        const x = ((hi & maskHi) << shiftHi) | ((lo & maskLo) >> shiftLo);
        const parity = @popCount(x) & 1;
        ret[bit/7] = x | parity;
    }
    return ret;
}

pub fn decodeMessage(allocator: mem.Allocator, message: []const u8) (mem.Allocator.Error || TransmissionError)![]u8 {
    if (message.len == 0) {
        return try allocator.alloc(u8, 0);
    }
    const n = (message.len * 8 - message.len) / 8;
    var ret = try allocator.alloc(u8, n);
    @memset(ret, 0);
    var bit: usize = 0;
    while (bit < ret.len * 8) : (bit += 7) {
        var x = message[bit/7];
        if (@popCount(x) & 1 != 0) {
            allocator.free(ret);
            return TransmissionError.WrongParity;
        }
        x &= 0xfe;
        const i = bit / 8;
        const j = i + 1;
        const shiftHi: u3 = @intCast(bit & 7);
        const shiftLo: u3 = @intCast(@min(7, 8 - (bit & 7)));
        ret[i] |= x >> shiftHi;
        if (j < ret.len) ret[j] |= x << shiftLo;
    }
    return ret;
}
