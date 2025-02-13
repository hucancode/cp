const Day = struct {
    name: []const u8,
    present: []const u8,
};

const days = [12]Day{
    Day{
        .name = "first",
        .present = "a Partridge in a Pear Tree",
    },
    Day{
        .name = "second",
        .present = "two Turtle Doves",
    },
    Day{
        .name = "third",
        .present = "three French Hens",
    },
    Day{
        .name = "fourth",
        .present = "four Calling Birds",
    },
    Day{
        .name = "fifth",
        .present = "five Gold Rings",
    },
    Day{
        .name = "sixth",
        .present = "six Geese-a-Laying",
    },
    Day{
        .name = "seventh",
        .present = "seven Swans-a-Swimming",
    },
    Day{
        .name = "eighth",
        .present = "eight Maids-a-Milking",
    },
    Day{
        .name = "ninth",
        .present = "nine Ladies Dancing",
    },
    Day{
        .name = "tenth",
        .present = "ten Lords-a-Leaping",
    },
    Day{
        .name = "eleventh",
        .present = "eleven Pipers Piping",
    },
    Day{
        .name = "twelfth",
        .present = "twelve Drummers Drumming",
    },
};
const std = @import("std");
const bufPrint = std.fmt.bufPrint;

pub fn recite(buffer: []u8, start_verse: u32, end_verse: u32) []const u8 {
    var n: usize = 0;
    for (start_verse - 1..end_verse) |i| {
        n += (bufPrint(buffer[n..], "On the {s} day of Christmas my true love gave to me: ", .{days[i].name}) catch buffer[0..0]).len;
        var j = i;
        while (j > 0) {
            n += (bufPrint(buffer[n..], "{s}, ", .{days[j].present}) catch buffer[0..0]).len;
            j -= 1;
        }
        if (i > 0) {
            n += (bufPrint(buffer[n..], "and {s}.\n", .{days[0].present}) catch buffer[0..0]).len;
        } else {
            n += (bufPrint(buffer[n..], "{s}.\n", .{days[0].present}) catch buffer[0..0]).len;
        }
    }
    return std.mem.trimRight(u8, buffer[0..n], "\n");
}
