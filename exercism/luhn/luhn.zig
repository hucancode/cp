pub fn isValid(s: []const u8) bool {
    var i: u32 = 0;
    var sum: u32 = 0;
    const n: usize = s.len;
    for (0..n) |j| {
        const c = s[n - 1 - j];
        if (c == ' ') {
            continue;
        }
        if (c < '0' or c > '9') {
            return false;
        }
        i += 1;
        if (i % 2 == 0) {
            const x = (c - '0') * 2;
            if (x > 9) {
                sum += x - 9;
            } else {
                sum += x;
            }
        } else {
            sum += c - '0';
        }
    }
    return sum % 10 == 0 and i > 1;
}
