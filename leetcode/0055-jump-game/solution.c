int max(int a, int b) {
    return a>b?a:b;
}

bool canJump(int* nums, int n) {
    int k = 0;
    for(int i = 0;i<n;i++) {
        if(i > k) {
            return false;
        }
        k = max(k, i+nums[i]);
    }
    return true;
}
