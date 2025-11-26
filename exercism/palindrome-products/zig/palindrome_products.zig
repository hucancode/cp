const std = @import("std");
const mem = std.mem;
const ArrayList = std.ArrayList;

pub const Pair = struct {
    first: u64,
    second: u64,
};

pub const Palindrome = struct {
    value: u64,
    factors: []Pair,
};

inline fn pow10(n: u32) u64 {
    return switch (n) {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        12 => 1_000_000_000_000,
        13 => 10_000_000_000_000,
        14 => 100_000_000_000_000,
        15 => 1_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        else => unreachable,
    };
}

inline fn digits(n: u64) u32 {
    var x = n;
    var d: u32 = 0;
    while (x != 0) : (x /= 10) d += 1;
    return d;
}

// 1234 -> 12344321 (even)
// 1234 -> 1234321 (odd)
inline fn makePalindrome(seed: u64, odd: bool) u64 {
    var n = seed;
    var p = seed;
    if (odd) n /= 10; // skip middle digit for odd-length
    while (n != 0) : (n /= 10) {
        p = p * 10 + (n % 10);
    }
    return p;
}

fn factorize(allocator: mem.Allocator, p: u64, min: u64, max: u64) mem.Allocator.Error!?Palindrome {
    var factors = ArrayList(Pair){};
    defer factors.deinit(allocator);
    const n = @as(u64, @intFromFloat(@sqrt(@as(f64, @floatFromInt(p)))));
    for (@max(min, p/max)..n+1) |i| {
        if (p % i == 0 and p/i <= max) {
            try factors.append(allocator, .{ .first = i, .second = p/i });
        }
    }
    if (factors.items.len == 0) {
        return null;
    }
    return .{
        .value = p,
        .factors = try factors.toOwnedSlice(allocator),
    };
}

fn makeAllPalindrome(allocator: mem.Allocator, min: u64, max: u64) mem.Allocator.Error!ArrayList(u64) {
    var ret = ArrayList(u64){};
    const minP = min * min;
    const maxP = max * max;
    const minD = digits(minP);
    const maxD = digits(maxP);
    const seedMin = pow10((minD + 1) / 2 - 1);
    const seedMax = pow10((maxD + 1) / 2) - 1;
    for (seedMin..seedMax+1) |seed| {
        var p = makePalindrome(seed, true);
        if (p >= minP and p <= maxP) {
            try ret.append(allocator, p);
        }
        p = makePalindrome(seed, false);
        if (p >= minP and p <= maxP) {
            try ret.append(allocator, p);
        }
    }
    return ret;
}


pub fn smallest(allocator: mem.Allocator, min: u64, max: u64) mem.Allocator.Error!?Palindrome {
    var candidates = try makeAllPalindrome(allocator, min, max);
    defer candidates.deinit(allocator);
    std.mem.sort(u64, candidates.items, {}, std.sort.asc(u64));
    for(candidates.items) |p| {
        if(try factorize(allocator, p, min, max)) |v| {
            return v;
        }
    }
    return null;
}

pub fn largest(allocator: mem.Allocator, min: u64, max: u64) mem.Allocator.Error!?Palindrome {    
    var candidates = try makeAllPalindrome(allocator, min, max);
    defer candidates.deinit(allocator);
    std.mem.sort(u64, candidates.items, {}, std.sort.desc(u64));
    for(candidates.items) |p| {
        if(try factorize(allocator, p, min, max)) |v| {
            return v;
        }
    }
    return null;
}
