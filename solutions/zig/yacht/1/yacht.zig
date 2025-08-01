const std = @import("std");
pub const Category = enum {
    ones,
    twos,
    threes,
    fours,
    fives,
    sixes,
    full_house,
    four_of_a_kind,
    little_straight,
    big_straight,
    choice,
    yacht,
};

pub fn score(dice: [5]u3, category: Category) u32 {
    var ret: u32 = 0;
    var sorted: [5]u3 = undefined;
    @memcpy(&sorted, &dice);
    std.mem.sort(u3, &sorted, {}, comptime std.sort.asc(u3));
    var sentinel: u3 = 7;
    switch(category) {
        Category.ones => {
            sentinel = 1;
        },
        Category.twos => {
            sentinel = 2;
        },
        Category.threes => {
            sentinel = 3;
        },
        Category.fours => {
            sentinel = 4;
        },
        Category.fives => {
            sentinel = 5;
        },
        Category.sixes => {
            sentinel = 6;
        },
        Category.full_house => {
            if (sorted[0] == sorted[1] and
                sorted[3] == sorted[4] and
                sorted[0] != sorted[4] and
               (sorted[2] == sorted[0] or sorted[2] == sorted[4])) {
                sentinel = 0;
            }
        },
        Category.four_of_a_kind => {
            if (sorted[0] == sorted[3] or sorted[1] == sorted[4]) {
                sentinel = sorted[1];
                if (sorted[0] == sorted[4]) {
                    sorted[0] = 0;
                }
            }
        },
        Category.little_straight => {
            if (sorted[0] == 1 and 
                sorted[1] == 2 and
                sorted[2] == 3 and
                sorted[3] == 4 and
                sorted[4] == 5) {
                ret = 30;
            }
        },
        Category.big_straight => {
            if (sorted[0] == 2 and
                sorted[1] == 3 and 
                sorted[2] == 4 and
                sorted[3] == 5 and
                sorted[4] == 6) {
                ret = 30;
            }
        },
        Category.choice => {
            sentinel = 0;
        },
        Category.yacht => {
            if (sorted[0] == sorted[4]) {
                ret = 50;
            }
        },
    }
    for(sorted) |x| {
        if (sentinel == 0 or x == sentinel) {
            ret += x;
        }
    }
    return ret;
}
