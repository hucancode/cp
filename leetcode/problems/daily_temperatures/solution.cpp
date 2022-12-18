class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        stack<int> st;
        int n = temperatures.size();
        vector<int> ret(n, 0);
        for(int i = 0;i<n;i++) {
            int x = temperatures[i];
            while(!st.empty() && temperatures[st.top()] < x) {
                auto j = st.top();
                ret[j] = i - j;
                st.pop();
            }
            st.push(i);
        }
        return ret;
    }
};