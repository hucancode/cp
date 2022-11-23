class Solution {
public:
    int maxArea(vector<int>& height) {
        int ret = 0;
        int n = height.size();
        vector<int> st;
        st.reserve(n);
        st.push_back(0);
        for(int i = 1;i<n;i++) {
            for(auto j: st) {
                int h = min(height[i], height[j]);
                int w = i - j;
                ret = max(ret, w*h);
            }
            if(height[i] > height[st.back()]) {
                st.push_back(i);
            }
        }
        return ret;
    }
};