int max(int a, int b) {
    return a>b?a:b;
}
int jump(int* nums, int n) {
    int jump = 0;
    int distance = 0;
    int next = 0;
    for(int i = 0;i<n;i++) {
        if(i > distance) {
            distance = next;
            jump++;
        }
        next = max(next, i+nums[i]);
    }
    return jump;
}
