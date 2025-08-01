const mem = @import("std").mem;

pub fn toRna(allocator: mem.Allocator, dna: []const u8) mem.Allocator.Error![]const u8 {
    const n = dna.len;
    var ret = try allocator.alloc(u8, n);
    for (0..n) |i| {
        ret[i] = switch(dna[i]) {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            else => '_',
        };
    }
    return ret;
}
