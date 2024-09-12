long long max(long long a, long long b) {
    return a>b?a:b;
}
long long maxArrayValue(int* nums, int n){
    long long max_sum = 0;
    long long sum = 0;
    for(int i = n-1;i>=0;i--) {
        if(nums[i] > sum) {
            max_sum = max(sum, max_sum);
            sum = nums[i];
        } else {
            sum += nums[i];
        }
    }
    return max(max_sum, sum);
}
