const ascii = @import("std").ascii;

pub const SeriesError = error{
    InvalidCharacter,
    NegativeSpan,
    InsufficientDigits,
};

pub fn largestProduct(digits: []const u8, span: i32) SeriesError!u64 {
    if (span < 0) {
        return SeriesError.NegativeSpan;
    }
    if (span == 0) {
        return 1;
    }
    if (span > digits.len) {
        return SeriesError.InsufficientDigits;
    }
    for (digits) |d| {
        if (!ascii.isDigit(d)) {
            return SeriesError.InvalidCharacter;
        }
    }
    const uspan = @as(usize, @intCast(span));
    const n = digits.len;
    var ret: u64 = 0;
    var x: u64 = 1;
    var zeroCount: usize = 0;
    for (0..n) |i| {
        const d = digits[i] - '0';
        if (d == 0) {
            zeroCount += 1;
        } else {
            x *= d;
        }
        if (i >= uspan) {
            const p = digits[i-uspan] - '0';
            if (p == 0) {
                zeroCount -= 1;
            } else {
                x /= p;
            }
        }
        if (zeroCount == 0 and i >= uspan-1) {
            ret = @max(ret, x);
        }
    }
    return ret;
}
