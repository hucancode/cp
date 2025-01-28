const std = @import("std");
pub const DnaError = error{ EmptyDnaStrands, UnequalDnaStrands };
pub fn compute(first: []const u8, second: []const u8) DnaError!usize {
    if (first.len == 0 or second.len == 0) {
        return DnaError.EmptyDnaStrands;
    }
    if (first.len != second.len) {
        return DnaError.UnequalDnaStrands;
    }
    const n = first.len;
    var ret: usize = 0;
    for (0..n) |i| {
        if (first[i] != second[i]) {
            ret += 1;
        }
    }
    return ret;
}
