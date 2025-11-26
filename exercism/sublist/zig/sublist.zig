const std = @import("std");
const mem = std.mem;

pub const Relation = enum {
    equal,
    sublist,
    superlist,
    unequal,
};

fn compare_equal(list_one: []const i32, list_two: []const i32) bool {
    const n = list_one.len;
    for(0..n) |i| {
        if (list_one[i] != list_two[i]) {
            return false;
        }
    }
    return true;
}

pub fn compare(list_one: []const i32, list_two: []const i32) Relation {
    if(list_one.len == list_two.len) {
        if (compare_equal(list_one, list_two)) {
            return .equal;
        } else {
            return .unequal;
        }
    }
    if (list_one.len > list_two.len) {
        const n = list_one.len-list_two.len;
        for(0..n+1) |i| {
            if (compare_equal(list_one[i..i+list_two.len], list_two)) {
                return .superlist;
            }
        }
        return .unequal;
    }
    const n = list_two.len-list_one.len;
    for(0..n+1) |i| {
        if (compare_equal(list_one, list_two[i..i+list_one.len])) {
            return .sublist;
        }
    }
    return .unequal;
}
