const std = @import("std");
const mem = std.mem;
const ArrayList = std.ArrayList;

pub fn prime(allocator: mem.Allocator, number: usize) !usize {
    var primes = ArrayList(usize).init(allocator);
    defer primes.deinit();
    var n: usize = 1;
    while (primes.items.len < number) {
        n += 1;
        var ok = true;
        for(primes.items) |x| {
            if(n%x == 0) {
                ok = false;
                break;
            }
        }
        if(ok) {
            try primes.append(n);
        }
    }
    return n;
}
