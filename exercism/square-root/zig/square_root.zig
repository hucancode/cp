pub fn squareRoot(radicand: usize) usize {
    const inputFloat = @as(f32, @floatFromInt(radicand));
    const retFloat = @sqrt(inputFloat);
    const retInt = @as(usize, @intFromFloat(retFloat));
    return retInt;
}
