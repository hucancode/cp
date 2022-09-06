class Solution {
public:
    int longestValidParentheses(string s) {
        int ret = 0;
        stack<int> st;
        vector<int> f(s.size(), 0);
        for(int i = 0;i<s.size();i++) {
            if(s[i] == '(') {
                st.push(i);
                continue;
            }
            if(st.size() > 0) {
                int j = st.top();
                st.pop();
                int len = i - j + 1;
                if(j > 0) {
                    len += f[j-1];
                }
                f[i] = len;
                ret = max(ret, f[i]);
            }
        }
        return ret;
    }
};