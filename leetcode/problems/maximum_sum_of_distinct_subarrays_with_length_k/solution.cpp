class Solution {
public:
    long long maximumSubarraySum(vector<int>& nums, int k) {
        map<int, int> st;
        int n = nums.size();
        long long sum = 0;
        long long ret = 0;
        for(int i = 0;i<k;i++) {
            st[nums[i]]++;
            sum += nums[i];
        }
        if(st.size() == k) {
            ret = max(ret, sum);
        }
        for(int i = k;i<n;i++) {
            auto a = nums[i-k];
            auto b = nums[i];
            st[a]--;
            st[b]++;
            sum += b-a;
            if(st[a] == 0) {
                st.erase(st.find(a));
            }
            if(st.size() < k) {
                continue;
            }
            ret = max(ret, sum);
        }
        return ret;
    }
};