class Solution {
public:
    int jump(vector<int>& nums) {
        const int INF = 10001;
        int n = nums.size();
        vector<int> f(n, INF);
        f[0] = 0;
        for(int i = 0;i<n;i++) {
            if(f[i] == INF) {
                continue;
            }
            int left = i+1;
            int right = min(n, left + nums[i]);
            for(int j = left;j<right;j++) {
                if(f[j] != INF) {
                    continue;
                }
                f[j] = f[i]+1;
            }
            if(right >= n) {
                break;
            }
        }
        return f[n-1];
    }
};