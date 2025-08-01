const std = @import("std");
const EnumSet = std.EnumSet;

pub const Allergen = enum {
    eggs,
    peanuts,
    shellfish,
    strawberries,
    tomatoes,
    chocolate,
    pollen,
    cats,
};
pub fn isAllergicTo(score: u8, allergen: Allergen) bool {
    return initAllergenSet(score).contains(allergen);
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    var ret = EnumSet(Allergen).initEmpty();
    const v = std.enums.values(Allergen);
    for(0..v.len) |i| {
        if ((@as(u8, 1)<<@as(u3, @intCast(i))) & score != 0) {
            ret.toggle(v[i]);
        }
    }
    return ret;
}
