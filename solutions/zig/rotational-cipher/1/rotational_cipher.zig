const mem = @import("std").mem;

pub fn rotate(allocator: mem.Allocator, text: []const u8, shiftKey: u5) mem.Allocator.Error![]u8 {
    const n = text.len;
    const ret = try allocator.alloc(u8, n);
    for (0..n) |i| {
        const c = text[i];
        if (c >= 'a' and c <= 'z') {
            ret[i] = (((c - 'a') + shiftKey) % 26) + 'a';
            continue;
        }
        if (c >= 'A' and c <= 'Z') {
            ret[i] = (((c - 'A') + shiftKey) % 26) + 'A';
            continue;
        }
        ret[i] = c;
    }
    return ret;
}
