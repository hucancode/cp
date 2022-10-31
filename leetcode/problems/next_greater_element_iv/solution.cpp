class Solution {
public:
    vector<int> secondGreaterElement(vector<int>& nums) {
        typedef pair<int, int> pi;
        int n = nums.size();
        vector<int> f(n, -1); // f[i] = first greater of i
        vector<int> g(n, -1); // g[i] = second greater of i
        stack<int> st;
        priority_queue<pi, vector<pi>, greater<pi>> st2;
        st.push(0);
        for(int i = 1;i<n;i++) {
            while(!st2.empty() && nums[i] > nums[st2.top().second]) {
                g[st2.top().second] = i;
                st2.pop();
            }
            while(!st.empty() && nums[i] > nums[st.top()]) {
                f[st.top()] = i;
                st2.emplace(nums[st.top()], st.top());
                st.pop();
            }
            st.push(i);
        }
        vector<int> ret(n, -1);
        for(int i = 0;i<n;i++) {
            if(g[i] == -1) {
                continue;
            }
            ret[i] = nums[g[i]];
        }
        return ret;
    }
};