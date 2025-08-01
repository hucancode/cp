const math = @import("std").math;
const std = @import("std");

fn fastPow(base: u128, exp: u32) !u128 {
    //std.debug.print("calculating {}^{}\n", .{ base, exp });
    if (exp == 0) {
        return 1;
    }
    if (base < 2) {
        return base;
    }
    var ret: u128 = 1;
    const k: u128 = math.maxInt(u128);
    var a = base;
    var n = exp;
    while (true) {
        if (n % 2 == 1) {
            if (a > 0 and ret > k / a) {
                return error.IntegerOverflow;
            }
            ret *= a;
        }
        n >>= 1;
        if (n == 0) {
            break;
        }
        if (a > math.sqrt(math.maxInt(u128))) {
            std.debug.print("integer overflow, last recognized value: {}\n", .{a});
            return error.IntegerOverflow;
        }
        a *= a;
    }
    return ret;
}

fn countDigits(num: u128) u32 {
    var ret: u32 = 0;
    var n = num;
    while (n > 0) {
        ret += 1;
        n /= 10;
    }
    return ret;
}

pub fn isArmstrongNumber(num: u128) bool {
    const n = countDigits(num);
    std.debug.print("number of digits: {}\n", .{n});
    var sum: u128 = 0;
    var x = num;
    while (x > 0) {
        const a = x % 10;
        const b = fastPow(a, n) catch return false;
        if (sum > math.maxInt(u128) - b) {
            return false;
        }
        sum += b;
        x /= 10;
    }
    return sum == num;
}
