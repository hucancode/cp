const std = @import("std");
const mem = std.mem;

pub const ConversionError = error{
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit,
};

pub fn convert(
    allocator: mem.Allocator,
    digits: []const u32,
    input_base: u32,
    output_base: u32,
) (mem.Allocator.Error || ConversionError)![]u32 {
    if (input_base < 2) {
        return ConversionError.InvalidInputBase;
    }
    if (output_base < 2) {
        return ConversionError.InvalidOutputBase;
    }
    var n: u32 = 0;
    for(digits) |x| {
        if (x >= input_base) {
            return ConversionError.InvalidDigit;
        }
        n *= input_base;
        n += x;
    }
    if (n == 0) {
        var ret: []u32 = try allocator.alloc(u32, 1);
        ret[0] = 0;
        return ret;
    }
    var len: usize = 0;
    var m = n;
    while (m > 0) {
        len += 1;
        m /= output_base;
    }
    var ret: []u32 = try allocator.alloc(u32, len);
    while (n > 0) {
        const x = n%output_base;
        ret[len-1] = x;
        len -= 1;
        n /= output_base;
    }
    return ret;
}
