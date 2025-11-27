const std = @import("std");
const mem = std.mem;

/// Returns the counts of the words in `s`.
/// Caller owns the returned memory.
pub fn countWords(allocator: mem.Allocator, s: []const u8) !std.StringHashMap(u32) {
    var ret = std.StringHashMap(u32).init(allocator);
    errdefer {
        var it = ret.keyIterator();
        while (it.next()) |key_ptr| {
            allocator.free(key_ptr.*);
        }
        ret.deinit();
    }
    var it = mem.tokenizeAny(u8, s, " ,.\n");
    while (it.next()) |k| {
        if (k.len == 0) continue;
        const sanitized = trimAround(k);
        if (sanitized.len == 0) continue;
        const normalized = try allocator.alloc(u8, sanitized.len);
        errdefer allocator.free(normalized);
        _ = std.ascii.lowerString(normalized, sanitized);
        const entry = try ret.getOrPut(normalized);
        if (entry.found_existing) {
            allocator.free(normalized); // not used
            entry.value_ptr.* += 1;
        } else {
            entry.value_ptr.* = 1;
        }
    }
    return ret;
}

fn trimAround(token: []const u8) []const u8 {
    const trimChars = "'\"()!&@$%^:";
    var start: u64 = 0;
    var end: u64 = token.len;
    while (start < end and mem.indexOf(u8, trimChars, token[start .. start + 1]) != null) {
        start += 1;
    }
    while (end > start and mem.indexOf(u8, trimChars, token[end - 1 .. end]) != null) {
        end -= 1;
    }
    return token[start..end];
}
