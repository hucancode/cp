const std = @import("std");
pub const HighScores = struct {
    top: [3]i32,
    last: i32,
    count: u3,
    pub fn init(scores: []const i32) HighScores {
        var ret = HighScores {
            .top = .{0, 0, 0},
            .last = 0,
            .count = 0,
        };
        for(scores) |x| {
            ret.last = x;
            var standby = x;
            ret.count = @min(ret.count + 1, 3);
            for (0..ret.count) |i| {
                if (standby > ret.top[i]) {
                    const tmp = standby;
                    standby = ret.top[i];
                    ret.top[i] = tmp;
                }
            }
        }
        return ret;
    }

    pub fn latest(self: *const HighScores) ?i32 {
        if(self.count == 0) {
            return null;
        }
        return self.last;
    }

    pub fn personalBest(self: *const HighScores) ?i32 {
        if(self.count == 0) {
            return null;
        }
        return self.top[0];
    }

    pub fn personalTopThree(self: *const HighScores) []const i32 {
        return self.top[0..self.count];
    }
};
