pub fn binarySearch(comptime T: type, target: T, items: []const T) ?usize {
    var l: usize = 0;
    var r: usize = items.len;
    while(l < r) {
        const m = l + (r-l)/2;
        const x = items[m];
        if (x == target) {
            return m;
        }
        if (x > target) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    return null;
}
