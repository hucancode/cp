pub fn squareOfSum(number: usize) usize {
    var n: usize = 0;
    for (0..number+1) |i| n += i;
    return n*n;
}

pub fn sumOfSquares(number: usize) usize {
    var n: usize = 0;
    for (0..number+1) |i| n += i*i;
    return n;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
