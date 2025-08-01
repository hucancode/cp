const mem = @import("std").mem;

pub fn isPangram(str: []const u8) bool {
    var vis: u32 = 0;
    for (str) |c| {
        var i: u5 = 0;
        const x: u32 = 1;
        if (c >= 'a' and c <= 'z') {
            i = @as(u5, @truncate(c - 'a'));
        }
        if (c >= 'A' and c <= 'Z') {
            i = @as(u5, @truncate(c - 'A'));
        }
        vis |= x << i;
    }
    return vis == (1 << 26) - 1;
}
