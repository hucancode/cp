const fmt = @import("std").fmt;
const ascii = @import("std").ascii;

pub fn encode(buffer: []u8, input: []const u8) []u8 {
    var i: usize = 0;
    var count: u32 = 0;
    var token: u8 = 'a';
    for (input) |c| {
        if (c == token) {
            count += 1;
        } else {
            if (count == 1) {
                buffer[i] = token;
                i += 1;
            } else if (count > 1) {
                const written = fmt.bufPrint(buffer[i..], "{d}{c}", .{ count, token }) catch unreachable;
                i += written.len;
            }
            token = c;
            count = 1;
        }
    }
    if (count == 1) {
        buffer[i] = token;
        i += 1;
    } else if (count > 1) {
        const written = fmt.bufPrint(buffer[i..], "{d}{c}", .{ count, token }) catch unreachable;
        i += written.len;
    }
    return buffer[0..i];
}

pub fn decode(buffer: []u8, input: []const u8) []u8 {
    var i: usize = 0;
    var count: u32 = 0;
    for (input) |c| {
        if (ascii.isDigit(c)) {
            const d = c - '0';
            count = count * 10 + d;
        } else {
            if (count == 0) {
                buffer[i] = c;
                i += 1;
            } else {
                for (i..i + count) |j| {
                    buffer[j] = c;
                }
                i += count;
            }
            count = 0;
        }
    }
    return buffer[0..i];
}
