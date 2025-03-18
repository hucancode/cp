int longestNiceSubarray(int* nums, int n) {
    int ret = 0;
    int j = 0;
    int sum = 0;
    for (int i = 0; i < n; i++) {
        while((nums[i] & sum) != 0) sum &= ~nums[j++];
        int score = i-j+1;
        if(score > ret) ret = score;
        sum |= nums[i];
    }
    return ret;
}
