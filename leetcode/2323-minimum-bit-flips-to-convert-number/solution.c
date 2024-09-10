int minBitFlips(int start, int goal) {
    int diff = start ^ goal;
    int ret = 0;
    while(diff) {
        ret += diff & 1;
        diff >>= 1;
    }
    return ret;
}
