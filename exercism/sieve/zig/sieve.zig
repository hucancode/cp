const std = @import("std");

pub fn primes(buffer: []u32, limit: u32) []u32 {
    var n: usize = 0;
    const allocator = std.heap.page_allocator;
    const isPrime = allocator.alloc(bool, limit+1) catch unreachable;
    @memset(isPrime, true);
    defer allocator.free(isPrime);
    for (2..limit+1) |i| {
        if (isPrime[i]) {
            buffer[n] = @as(u32, @truncate(i));
            n += 1;
            const j = limit/i+1;
            if (j < 2) {
                continue;
            }
            for (2..j) |k| {
                isPrime[i*k] = false;
            }
        }
    }
    return buffer[0..n];
}
