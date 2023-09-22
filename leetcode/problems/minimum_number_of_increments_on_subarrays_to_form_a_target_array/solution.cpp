class Solution {
public:
    int minNumberOperations(vector<int>& target) {
        stack<int> st;
        int ret = 0;
        for(auto x: target) {
            if(!st.empty() && st.top() >= x) ret += st.top() - x;
            while(!st.empty() && st.top() >= x) st.pop();
            st.push(x);
        }
        if(!st.empty()) {
            ret += st.top();
        }
        return ret;
    }
};