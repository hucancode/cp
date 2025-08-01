pub fn isValidIsbn10(s: []const u8) bool {
    var k: u8 = 10;
    var sum: u32 = 0;
    for (s) |c| {
        if (k == 0) {
            return false;
        }
        if (c == '-') {
            continue;
        }
        if (c == 'X' and k == 1) {
            sum += 10;
            k -= 1;
            continue;
        }
        if (c >= '0' and c <= '9') {
            sum += (c - '0') * k;
            k -= 1;
            continue;
        }
        return false;
    }
    return k == 0 and sum % 11 == 0;
}
