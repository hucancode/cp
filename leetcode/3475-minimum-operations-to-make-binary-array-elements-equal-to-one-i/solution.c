int minOperations(int* nums, int n) {
    int f[n] = {};
    int ret = 0;
    for(int i = 0;i<n-2;i++) {
        if ((nums[i] + f[i])%2) continue;
        f[i] += 1;
        f[i+1] += 1;
        f[i+2] += 1;
        ret += 1;
    }
    if ((f[n-1] + nums[n-1])%2 && 
        (f[n-2] + nums[n-2])%2) {
        return ret;
    }
    return -1;
}
