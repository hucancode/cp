pub fn modifier(score: i8) i8 {
    return @divFloor(score - 10, 2);
}

pub fn ability() i8 {
    return 3;
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
        var ret = Character {
            .strength = 3,
            .dexterity = 3,
            .constitution = 3,
            .intelligence = 3,
            .wisdom = 3,
            .charisma = 3,
            .hitpoints = 3,
        };
        ret.hitpoints = 10 + modifier(ret.constitution);
        return ret;
    }
};
