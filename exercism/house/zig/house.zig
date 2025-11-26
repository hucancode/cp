const std = @import("std");
const bufPrint = std.fmt.bufPrint;

const Subject = struct {
    name: []const u8,
    verb: []const u8,
    
    pub fn intro(self: Subject, buffer: []u8) usize {
        return (bufPrint(buffer, "This is the {s}", .{ self.name }) catch buffer[0..0]).len;
    }
    pub fn act(self: Subject, buffer: []u8, object: []const u8) usize {
        return (bufPrint(buffer, " that {s} the {s}", .{ self.verb, object }) catch buffer[0..0]).len;
    }
    pub fn outro(buffer: []u8) usize {
        return (bufPrint(buffer, ".\n", .{}) catch buffer[0..0]).len;
    }
};

const verses = [_] Subject {
    Subject {
        .name = "house that Jack built",
        .verb = ""
    },
    Subject {
        .name = "malt",
        .verb = "lay in"
    },
    Subject {
        .name = "rat",
        .verb = "ate"
    },
    Subject {
        .name = "cat",
        .verb = "killed"
    },
    Subject {
        .name = "dog",
        .verb = "worried"
    },
    Subject {
        .name = "cow with the crumpled horn",
        .verb = "tossed"
    },
    Subject {
        .name = "maiden all forlorn",
        .verb = "milked"
    },
    Subject {
        .name = "man all tattered and torn",
        .verb = "kissed"
    },
    Subject {
        .name = "priest all shaven and shorn",
        .verb = "married"
    },
    Subject {
        .name = "rooster that crowed in the morn",
        .verb = "woke"
    },
    Subject {
        .name = "farmer sowing his corn",
        .verb = "kept"
    },
    Subject {
        .name = "horse and the hound and the horn",
        .verb = "belonged to"
    },
};

pub fn recite(buffer: []u8, start_verse: u32, end_verse: u32) []const u8 {
    var n: usize = 0;
    for(start_verse-1..end_verse) |i| {
        n += verses[i].intro(buffer[n..]);
        defer n += Subject.outro(buffer[n..]);
        if (i == 0) {
            continue;
        }
        for(0..i) |j| {
            const k = i-j;
            n += verses[k].act(buffer[n..], verses[k-1].name);
        }
    }
    return std.mem.trimRight(u8, buffer[0..n], "\n");

}
