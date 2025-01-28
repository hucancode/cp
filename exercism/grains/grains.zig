pub const ChessboardError = error{IndexOutOfBounds};
pub fn square(index: usize) ChessboardError!u64 {
    if (index == 0 or index > 64) {
        return ChessboardError.IndexOutOfBounds;
    }
    const i: u6 = @as(u6, @truncate(index - 1));
    const x: u64 = 1;
    return x << i;
}

pub fn total() u64 {
    return (1 << 64) - 1;
}
