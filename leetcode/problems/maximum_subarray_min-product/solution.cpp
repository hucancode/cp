class Solution {
public:
    int maxSumMinProduct(vector<int>& nums) {
        int n = nums.size();
        vector<long long> prefix(n+1, 0);
        for(int i =1;i<=n;i++) {
            prefix[i] = prefix[i-1]+nums[i-1];
        }
        vector<long long> sum(n, 0);
        vector<int> st;
        for(int i=0;i<n;i++) {
            int x = nums[i];
            while(!st.empty() && nums[st.back()] >= x) {
                st.pop_back();
            }
            int j = st.empty()?0:(st.back()+1);
            st.push_back(i);
            sum[i] += prefix[i+1] - prefix[j];
        }
        st.clear();
        for(int i=n-1;i>=0;i--) {
            int x = nums[i];
            while(!st.empty() && nums[st.back()] >= x) {
                st.pop_back();
            }
            int j = st.empty()?n:st.back();
            st.push_back(i);
            sum[i] += prefix[j] - prefix[i+1];
        }
        long long ret = 0;
        for(int i=0;i<n;i++) {
            long long score = sum[i]*nums[i];
            ret = max(ret, score);
        }
        const int MOD = 1e9+7;
        return ret%MOD;
    }
};