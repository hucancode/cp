pub fn eggCount(number: usize) usize {
    var ret: usize = 0;
    var n = number;
    while (n != 0) {
        ret += n & 1;
        n >>= 1; 
    }
    return ret;
}
