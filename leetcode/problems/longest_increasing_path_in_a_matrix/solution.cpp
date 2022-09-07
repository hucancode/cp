class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& a) {
        int m = a.size();
        int n = a[0].size();
        int ret = 0;
        vector<vector<int>> f(m, vector<int>(n, 1));
        map<int, vector<pair<int, int>>> mp;
        for(int i = 0;i<m;i++) {
            for(int j = 0;j<n;j++) {
                mp[a[i][j]].push_back(make_pair(i, j));
            }
        }
        for(const auto& items: mp) {
            //auto value = items.first;
            //cout<<"update all cells with value "<<value<<endl;
            for(const auto& x: items.second) {
                auto i = x.first;
                auto j = x.second;
                if(i<m-1 && a[i+1][j] > a[i][j]) {
                    f[i+1][j] = max(f[i+1][j], f[i][j] + 1);
                }
                if(i>0 && a[i-1][j] > a[i][j]) {
                    f[i-1][j] = max(f[i-1][j], f[i][j] + 1);
                }
                if(j<n-1 && a[i][j+1] > a[i][j]) {
                    f[i][j+1] = max(f[i][j+1], f[i][j] + 1);
                }
                if(j>0 && a[i][j-1] > a[i][j]) {
                    f[i][j-1] = max(f[i][j-1], f[i][j] + 1);
                }
                ret = max(ret, f[i][j]);
                if(ret == mp.size()) {
                    break;
                }
            }
            //cout<<"ret = "<<ret<<endl;
        }
        return ret;
    }
};