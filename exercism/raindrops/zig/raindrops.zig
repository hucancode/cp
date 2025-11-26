const bufPrint = @import("std").fmt.bufPrint;

pub fn convert(buffer: []u8, n: u32) []const u8 {
    var i: usize = 0;
    if (n%3 == 0) {
        i += (bufPrint(buffer[i..], "{s}", .{"Pling"}) catch buffer[0..0]).len;
    }
    if (n%5 == 0) {
        i += (bufPrint(buffer[i..], "{s}", .{"Plang"}) catch buffer[0..0]).len;
    }
    if (n%7 == 0) {
        i += (bufPrint(buffer[i..], "{s}", .{"Plong"}) catch buffer[0..0]).len;
    }
    if (i == 0) {
        i += (bufPrint(buffer[i..], "{d}", .{n}) catch buffer[0..0]).len;
    }
    return buffer[0..i];
}
