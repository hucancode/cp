const std = @import("std");

pub const ComputationError = error{ IllegalArgument, IntegerOverflow };

pub fn steps(number: usize) anyerror!usize {
    if (number == 0) {
        return ComputationError.IllegalArgument;
    }
    var ret: usize = 0;
    var n: usize = number;
    while (n != 1) {
        ret += 1;
        if (n % 2 == 0) {
            n /= 2;
        } else {
            if (n > (std.math.maxInt(usize) - 1) / 3) {
                return ComputationError.IntegerOverflow;
            }
            n = 3 * n + 1;
        }
    }
    return ret;
}
