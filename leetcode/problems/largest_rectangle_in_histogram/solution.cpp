class Solution {
public:
    int largestRectangleArea(vector<int>& a) {
        stack<int> st;
        int n = a.size();
        int ret = 0;
        for(int i = 0; i <= n; i++){
            bool goDown = i == n || (!st.empty() && a[st.top()] > a[i]);
            while(!st.empty() && goDown){
                int height = a[st.top()];
                st.pop();
                int r = i - 1;
                int l = st.empty() ? 0: (st.top() + 1);
                ret = max(ret, (r - l + 1) * height);
                goDown = i == n || (!st.empty() && a[st.top()] > a[i]);
            }
            st.push(i);
        }
        return ret;
    }
};