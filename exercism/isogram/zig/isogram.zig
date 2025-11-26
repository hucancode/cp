const mem = @import("std").mem;

pub fn isIsogram(str: []const u8) bool {
    var vis = mem.zeroes([26]bool);
    for (str) |c| {
        if (c == '-' or c == ' ') {
            continue;
        }
        var i: u5 = 0;
        if (c >= 'a' and c <= 'z') {
            i = @as(u5, @truncate(c - 'a'));
        }
        if (c >= 'A' and c <= 'Z') {
            i = @as(u5, @truncate(c - 'A'));
        }
        if (vis[i]) {
            return false;
        }
        vis[i] = true;
    }
    return true;
}
