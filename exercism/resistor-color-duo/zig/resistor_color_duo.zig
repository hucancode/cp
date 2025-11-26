const std = @import("std");

pub const ColorBand = enum(usize) {
    black = 0,
    brown = 1,
    red = 2,
    orange = 3,
    yellow = 4,
    green = 5,
    blue = 6,
    violet = 7,
    grey = 8,
    white = 9,
};
pub fn colorCode(colors: [2]ColorBand) usize {
    const i = @intFromEnum(colors[0]);
    const j = @intFromEnum(colors[1]);
    return i * 10 + j;
}
