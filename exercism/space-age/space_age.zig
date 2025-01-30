const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
const ORBITAL_PERIOD = [_]f64{ 0.2408467, 0.61519726, 1.0, 1.8808158, 11.862615, 29.447498, 84.016846, 164.79132 };
pub const Planet = enum(usize) {
    mercury = 0,
    venus = 1,
    earth = 2,
    mars = 3,
    jupiter = 4,
    saturn = 5,
    uranus = 6,
    neptune = 7,

    pub fn age(self: Planet, seconds: usize) f64 {
        return @as(f64, @floatFromInt(seconds)) / (EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD[@intFromEnum(self)]);
    }
};
