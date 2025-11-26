const std = @import("std");
var prng = std.rand.DefaultPrng.init(0);
const rand = prng.random();

pub fn modifier(score: i8) i8 {
    return @divFloor(score - 10, 2);
}

pub fn ability() i8 {
    var candidates = [_]i8 {
        rand.intRangeAtMost(i8, 1, 6),
        rand.intRangeAtMost(i8, 1, 6),
        rand.intRangeAtMost(i8, 1, 6),
        rand.intRangeAtMost(i8, 1, 6),
    };
    std.mem.sort(i8, &candidates, {}, comptime std.sort.desc(i8));
    return candidates[0] + candidates[1] + candidates[2];
}

pub const Character = struct {
    strength: i8,
    dexterity: i8,
    constitution: i8,
    intelligence: i8,
    wisdom: i8,
    charisma: i8,
    hitpoints: i8,

    pub fn init() Character {
        var ret: Character = undefined;
        ret.strength = ability();
        ret.dexterity = ability();
        ret.constitution = ability();
        ret.intelligence = ability();
        ret.wisdom = ability();
        ret.charisma = ability();
        ret.hitpoints = 10 + modifier(ret.constitution);
        return ret;
    }
};
