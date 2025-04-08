int minimumOperations(int* nums, int n) {
    int vis[101] = {};
    int i = n-1;
    for(;i>=0 && !vis[nums[i]];i--) {
        vis[nums[i]] = true;
    }
    return (i+3)/3;
}
