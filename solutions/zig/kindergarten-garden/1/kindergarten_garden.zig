const mem = @import("std").mem;

pub const Plant = enum {
    clover,
    grass,
    radishes,
    violets,
};

pub fn plants(diagram: []const u8, student: []const u8) [4]Plant {
    const students = [_][]const u8{
        "Alice", "Bob",  "Charlie",  "David",  "Eve",  "Fred",  "Ginny",  "Harriet",  "Ileana",  "Joseph",  "Kincaid",  "Larry"
    };
    var tasks: [12][4]Plant = undefined;
    var i: usize = 0;
    var j: usize = 0;
    for(diagram) |c| {
        if(c == '\n') {
            i = 0;
            j += 1;
            continue;
        }
        tasks[i/2][j*2+i%2] = switch(c) {
            'C' => .clover,
            'G' => .grass,
            'R' => .radishes,
            else => .violets,
        };
        i += 1;
    }
    for(0..12) |k| {
        if(mem.eql(u8, student, students[k])) {
            return tasks[k];
        }
    }
    return tasks[0];
}
