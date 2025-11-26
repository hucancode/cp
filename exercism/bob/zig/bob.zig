const ascii = @import("std").ascii;
const mem = @import("std").mem;
pub fn response(s: []const u8) []const u8 {
    const normalized = mem.trim(u8, s, "\r\n\t ");
    if (normalized.len == 0) {
        return "Fine. Be that way!";
    }
    const isQuestion = normalized[normalized.len - 1] == '?';
    var asciiCount: u32 = 0;
    var capitalCount: u32 = 0;
    for (normalized) |c| {
        if (ascii.isAlphabetic(c)) {
            asciiCount += 1;
            if (ascii.isUpper(c)) {
                capitalCount += 1;
            }
        }
    }
    const isYelling = asciiCount > 0 and capitalCount == asciiCount;
    if (isQuestion and isYelling) {
        return "Calm down, I know what I'm doing!";
    }
    if (isQuestion) {
        return "Sure.";
    }
    if (isYelling) {
        return "Whoa, chill out!";
    }
    return "Whatever.";
}
