const mem = @import("std").mem;
pub fn isPangram(str: []const u8) bool {
    var vis = mem.zeroes([26]bool);
    for (str) |c| {
        if (c >= 'a' and c <= 'z') {
            vis[c - 'a'] = true;
        }
        if (c >= 'A' and c <= 'Z') {
            vis[c - 'A'] = true;
        }
    }
    for (vis) |v| {
        if (!v) {
            return false;
        }
    }
    return true;
}
