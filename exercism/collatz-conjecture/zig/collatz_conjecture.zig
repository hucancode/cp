pub const ComputationError = error{
    IllegalArgument,
};

pub fn steps(number: usize) anyerror!usize {
    if (number == 0) {
        return ComputationError.IllegalArgument;
    }
    var ret: usize = 0;
    var n: usize = number;
    while (n != 1) {
        ret += 1;
        if (n % 2 == 0) {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    return ret;
}
