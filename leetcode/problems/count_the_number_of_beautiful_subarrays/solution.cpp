class Solution {
public:
    long long beautifulSubarrays(vector<int>& nums) {
        int n = nums.size();
        vector<int> compat(n+1, 0);
        map<int, long long> group;
        group[0] = 1;
        for(int i = 1;i<=n;i++) {
            compat[i] = compat[i-1] ^ nums[i-1];
            group[compat[i]]++;
            //cout<<bitset<10>(compat[i])<<" ";
        }
        long long ret = 0;
        for(auto& item:group) {
            auto x = item.second;
            ret += x*(x-1)/2;
        }
        return ret;
    }
};