pub const Coordinate = struct {
    x: f32,
    y: f32,
    pub fn init(x: f32, y: f32) Coordinate {
        return Coordinate{ .x = x, .y = y };
    }
    pub fn score(self: Coordinate) usize {
        const lengthSquared = self.x * self.x + self.y * self.y;
        if (lengthSquared > 100.0) {
            return 0;
        }
        if (lengthSquared > 25.0) {
            return 1;
        }
        if (lengthSquared > 1.0) {
            return 5;
        }
        return 10;
    }
};
