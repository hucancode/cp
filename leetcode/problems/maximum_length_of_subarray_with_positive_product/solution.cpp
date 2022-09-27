class Solution {
public:
    int getMaxLen(vector<int>& nums) {
        int n = nums.size();
        vector<int> f(n+1, 0);
        vector<int> g(n+1, 0);
        int ret = 0;
        for(int i = 1;i<=n;i++) {
            if(nums[i-1] > 0) {
                f[i] = f[i-1] + 1;
                g[i] = g[i-1] > 0?(g[i-1] + 1):0;
            } else if(nums[i-1] < 0) {
                f[i] = g[i-1] > 0?(g[i-1] + 1):0;
                g[i] = f[i-1] + 1;
            } else {
                f[i] = 0;
                g[i] = 0;
            }
            ret = max(ret, f[i]);
        }
        // cout<<"a: _ ";
        // for(const auto& x:nums) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        // cout<<"f: ";
        // for(const auto& x:f) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        // cout<<"g: ";
        // for(const auto& x:g) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        return ret;
    }
};