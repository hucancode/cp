class Solution {
public:
    bool dfs(vector<vector<int>>& mat, int day) {
        //cout<<"dfs day "<<day<<endl;
        int n = mat.size();
        int m = mat[0].size();
        stack<pair<int,int>> st;
        for(int i = 0;i<m; i++) {
            if(mat[0][i] == 0 || mat[0][i] > day) {
                st.emplace(0, i);
            }
        }
        vector<vector<bool>> vis(n, vector<bool>(m, false));
        auto valid = [&](int i,  int j) {
            return i >= 0 && i < n && j >=0 && j < m && 
                (mat[i][j] == 0 || mat[i][j] > day) &&
                !vis[i][j];
        };
        int x,y;
        while(!st.empty()) {
            tie(x,y) = st.top();
            st.pop();
            //cout<<"pop "<<x<<"-"<<y<<endl;
            vis[x][y] = true;
            if(x == n - 1) {
                return true;
            }
            for(auto i: {x-1, x+1}) {
                if(valid(i,y)) {
                    //cout<<"push "<<i<<"-"<<y<<endl;
                    st.emplace(i,y);
                }
            }
            for(auto j: {y-1, y+1}) {
                if(valid(x,j)) {
                    //cout<<"push "<<x<<"-"<<j<<endl;
                    st.emplace(x,j);
                }
            }
        }
        return false;
    }
    int latestDayToCross(int row, int col, vector<vector<int>>& cells) {
        vector<vector<int>> mat(row, vector<int>(col, 0));
        for(int i = 0;i<cells.size();i++) {
            auto cell = cells[i];
            auto x = cell[0]-1;
            auto y = cell[1]-1;
            mat[x][y] = i+1;
        }
        int l = 1, r = cells.size();
        while(l < r) {
            int m = (l+r)/2;
            bool good = dfs(mat, m);
            //cout<<"binary search "<<l<<"-"<<r<<" m "<<m<<" good = "<<good<<endl;
            if(good) {
                l = m+1;
            } else {
                r = m;
            }
        }
        return l-1;
    }
};