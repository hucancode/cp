const std = @import("std");
const mem = std.mem;
const ascii = std.ascii;

/// Encodes `s` using the Atbash cipher. Caller owns the returned memory.
pub fn encode(allocator: mem.Allocator, s: []const u8) mem.Allocator.Error![]u8 {
    var n: usize = 0;
    for(s) |c| {
        if (ascii.isAlphanumeric(c)) {
            n += 1;
        }
    }
    if (n == 0) {
        return try allocator.alloc(u8, 0);
    }
    var ret = try allocator.alloc(u8, n + (n-1)/5);
    var i: usize = 0;
    for(s) |c| {
        if (!ascii.isAlphanumeric(c)) {
            continue;
        }
        if ((i+1)%6 == 0) {
            ret[i] = ' ';
            i += 1;
        }
        if (ascii.isDigit(c)) {
            ret[i] = c;
        } else {
            ret[i] = 'z' - ascii.toLower(c) + 'a';
        }
        i += 1;
    }
    return ret;
}

/// Decodes `s` using the Atbash cipher. Caller owns the returned memory.
pub fn decode(allocator: mem.Allocator, s: []const u8) mem.Allocator.Error![]u8 {
    var n: usize = 0;
    for(s) |c| {
        if (ascii.isAlphanumeric(c)) {
            n += 1;
        }
    }
    var ret = try allocator.alloc(u8, n);
    var i: usize = 0;
    for(s) |c| {
        if (!ascii.isAlphanumeric(c)) {
            continue;
        }
        if (ascii.isDigit(c)) {
            ret[i] = c;
        } else {
            ret[i] = 'z' - ascii.toLower(c) + 'a';
        }
        i += 1;
    }
    return ret;
}
