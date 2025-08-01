const std = @import("std");
const mem = std.mem;

pub fn isBalanced(allocator: mem.Allocator, s: []const u8) !bool {
    var n: usize = 0;
    var stack = try allocator.alloc(u8, s.len);
    defer allocator.free(stack);
    for(s) |c| {
        if (c == '[' or c == '{' or c == '(') {
            stack[n] = c;
            n += 1;
        } else if (c == ']') {
            if (n <= 0 or stack[n-1] != '[') {
                return false;
            }
            n -= 1;
        } else if (c == '}') {
            if (n <= 0 or stack[n-1] != '{') {
                return false;
            }
            n -= 1;
        } else if (c == ')') {
            if (n <= 0 or stack[n-1] != '(') {
                return false;
            }
            n -= 1;
        }
    }
    return n == 0;
}
