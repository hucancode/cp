class Solution {
public:
    int numberOfArithmeticSlices(vector<int>& nums) {
        int n = nums.size();
        vector<int> f(n, 0);
        vector<int> delta(n);
        for(int i = 1;i<n;i++) {
            delta[i] = nums[i] - nums[i-1];
        }
        int ret = 0;
        for(int i = 2;i<n;i++) {
            if(delta[i] != delta[i-1]) {
                ret += f[i-1];
                f[i] = 0;
                continue;
            }
            if(f[i-1] == 0) {
                f[i] = 1;
                continue;
            }
            f[i] = f[i-1]*2-f[i-2]+1;
        }
        // cout<<"d=";
        // for(const auto&x:delta) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        // cout<<"f=";
        // for(const auto&x:f) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        ret += f[n-1];
        return ret;
    }
};