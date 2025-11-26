const std = @import("std");
const ascii = std.ascii;
const mem = std.mem;

pub fn clean(phrase: []const u8) ?[10]u8 {
    var ret: [10]u8 = undefined;
    var n: usize = 0;
    var started = false;
    for (phrase) |c| {
        if (mem.indexOfScalar(u8, " .-+()", c) != null) {
            continue;
        }
        if (!ascii.isDigit(c)) {
            return null;
        }
        if (n >= 10) {
            return null;
        }
        if (!started and c == '1') {
            started = true;
            continue;
        }
        if ((n == 0 or n == 3) and c <= '1') {
            return null;
        }
        ret[n] = c;
        n += 1;
    }
    if (n != 10) {
        return null;
    }
    return ret;
}
