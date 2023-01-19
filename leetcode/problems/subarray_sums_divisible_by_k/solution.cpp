class Solution {
public:
    int subarraysDivByK(vector<int>& nums, int k) {
        int n = nums.size();
        vector<int> prefix(n+1, 0);
        for(int i = 1;i<=n;i++) {
            prefix[i] = prefix[i-1]+nums[i-1];
        }
        int ret = 0;
        vector<int> remainderCount(k, 0);
        for(int i = 1;i<=n;i++) {
            int remainder = (k + (prefix[i] % k)) % k;// double mod to avoid negative remainder
            ret += remainderCount[remainder];
            remainderCount[remainder]++;
        }
        ret += remainderCount[0];
        return ret;
    }
};