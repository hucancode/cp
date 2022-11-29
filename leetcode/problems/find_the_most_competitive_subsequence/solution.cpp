class Solution {
public:
    vector<int> mostCompetitive(vector<int>& nums, int k) {
        int n = nums.size();
        stack<int> st;
        st.push(nums[0]);
        for(int i = 1;i<n;i++) {
            int left = n-i;
            while(!st.empty() && st.top() > nums[i] && st.size() + left > k) {
                st.pop();
            }
            st.push(nums[i]);
        }
        vector<int> ret;
        while(!st.empty()) {
            ret.push_back(st.top());
            st.pop();
        }
        reverse(ret.begin(), ret.end());
        ret.resize(k);
        return ret;
    }
};