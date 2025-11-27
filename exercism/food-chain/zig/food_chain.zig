const std = @import("std");
const Animal = struct {
    name: []const u8,
    name_long: ?[]const u8 = null,
    phrase: []const u8,
    next: ?*const Animal = null,
};

const fly = Animal{ .name = "fly", .phrase = "I don't know why she swallowed the fly. Perhaps she'll die." };
const spider = Animal{ .name = "spider", .name_long = "spider that wriggled and jiggled and tickled inside her", .phrase = "It wriggled and jiggled and tickled inside her.", .next = &fly };
const bird = Animal{ .name = "bird", .phrase = "How absurd to swallow a bird!", .next = &spider };
const cat = Animal{ .name = "cat", .phrase = "Imagine that, to swallow a cat!", .next = &bird };
const dog = Animal{ .name = "dog", .phrase = "What a hog, to swallow a dog!", .next = &cat };
const goat = Animal{ .name = "goat", .phrase = "Just opened her throat and swallowed a goat!", .next = &dog };
const cow = Animal{ .name = "cow", .phrase = "I don't know how she swallowed a cow!", .next = &goat };
const horse = Animal{ .name = "horse", .phrase = "She's dead, of course!" };
const verses = [_]Animal{ fly, spider, bird, cat, dog, goat, cow, horse };

fn append(buffer: []u8, i: *usize, str: []const u8) void {
    std.mem.copyForwards(u8, buffer[i.*..], str);
    i.* += str.len;
}
fn generate_verse(i: usize, buffer: []u8, idx: *usize) void {
    var animal: ?*const Animal = &verses[i];
    append(buffer, idx, "I know an old lady who swallowed a ");
    append(buffer, idx, animal.?.name);
    append(buffer, idx, ".\n");
    if (animal.?.next != null) {
        append(buffer, idx, animal.?.phrase);
        append(buffer, idx, "\n");
    }
    while (animal != null) {
        const node = animal.?;
        if (node.next == null) {
            append(buffer, idx, node.phrase);
            append(buffer, idx, "\n");
        } else {
            const next = node.next.?;
            append(buffer, idx, "She swallowed the ");
            append(buffer, idx, node.name);
            append(buffer, idx, " to catch the ");
            if (next.name_long != null) {
                append(buffer, idx, next.name_long.?);
            } else {
                append(buffer, idx, next.name);
            }
            append(buffer, idx, ".\n");
        }
        animal = node.next;
    }
}
pub fn recite(buffer: []u8, i: usize, j: usize) ![]const u8 {
    var idx: usize = 0;
    for (i - 1..j) |k| {
        generate_verse(k, buffer, &idx);
        append(buffer, &idx, "\n");
    }
    return std.mem.trimRight(u8, buffer[0..idx], "\n");
}