const std = @import("std");
pub const HighScores = struct {
    top: [3]i32 = .{0, 0, 0},
    last: ?i32 = null,
    count: u3 = 0,
    pub fn init(scores: []const i32) HighScores {
        var ret = HighScores {};
        for(scores) |x| {
            ret.last = x;
            var k = x;
            ret.count = @min(ret.count + 1, 3);
            for (0..ret.count) |i| if (k > ret.top[i]) std.mem.swap(i32, &k, &ret.top[i]);
        }
        return ret;
    }

    pub fn latest(self: *const HighScores) ?i32 {
        return self.last;
    }

    pub fn personalBest(self: *const HighScores) ?i32 {
        if(self.count == 0) return null;
        return self.top[0];
    }

    pub fn personalTopThree(self: *const HighScores) []const i32 {
        return self.top[0..self.count];
    }
};
