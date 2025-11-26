const math = @import("std").math;

pub const Classification = enum {
    deficient,
    perfect,
    abundant,
};

pub fn classify(n: u64) Classification {
    if (n <= 2) {
        return Classification.deficient;
    }
    const k = math.sqrt(n) + 1;
    var sum: u64 = 1;
    for (2..k) |i| {
        if (n % i == 0) {
            sum += i;
            const j = n / i;
            if (j != i) {
                sum += j;
            }
        }
    }
    if (sum > n) {
        return Classification.abundant;
    }
    if (sum < n) {
        return Classification.deficient;
    }
    return Classification.perfect;
}