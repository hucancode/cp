class Solution {
public:
    vector<vector<int>> pacificAtlantic(vector<vector<int>>& heights) {
        int n = heights.size();
        int m = heights[0].size();
        vector<vector<bool>> left(n, vector<bool>(m, false));
        vector<vector<bool>> right(n, vector<bool>(m, false));
        stack<int> st;
        for(int i = 0;i<n;i++) {
            st.push(i);
            st.push(0);
        }
        for(int j = 0;j<m;j++) {
            st.push(0);
            st.push(j);
        }
        while(!st.empty()) {
            int j = st.top();
            st.pop();
            int i = st.top();
            st.pop();
            left[i][j] = true;
            if(i > 0 && !left[i-1][j] && heights[i-1][j] >= heights[i][j]) {
                st.push(i-1);
                st.push(j);
            }
            if(i < n-1 && !left[i+1][j] && heights[i+1][j] >= heights[i][j]) {
                st.push(i+1);
                st.push(j);
            }
            if(j > 0 && !left[i][j-1] && heights[i][j-1] >= heights[i][j]) {
                st.push(i);
                st.push(j-1);
            }
            if(j < m-1 && !left[i][j+1] && heights[i][j+1] >= heights[i][j]) {
                st.push(i);
                st.push(j+1);
            }
        }
        for(int i = 0;i<n;i++) {
            st.push(i);
            st.push(m-1);
        }
        for(int j = 0;j<m;j++) {
            st.push(n-1);
            st.push(j);
        }
        while(!st.empty()) {
            int j = st.top();
            st.pop();
            int i = st.top();
            st.pop();
            right[i][j] = true;
            if(i > 0 && !right[i-1][j] && heights[i-1][j] >= heights[i][j]) {
                st.push(i-1);
                st.push(j);
            }
            if(i < n-1 && !right[i+1][j] && heights[i+1][j] >= heights[i][j]) {
                st.push(i+1);
                st.push(j);
            }
            if(j > 0 && !right[i][j-1] && heights[i][j-1] >= heights[i][j]) {
                st.push(i);
                st.push(j-1);
            }
            if(j < m-1 && !right[i][j+1] && heights[i][j+1] >= heights[i][j]) {
                st.push(i);
                st.push(j+1);
            }
        }
        vector<vector<int>> ret;
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                if(left[i][j] && right[i][j]) {
                    ret.push_back({i, j});
                }
            }
        }
        return ret;
    }
};