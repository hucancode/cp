class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>>& a) {
        int n = a.size();
        int m = a[0].size();
        vector<vector<bool>> visited(n, vector<bool>(m, false));
        int i = 0;
        int j = 0;
        int di = 0;
        int dj = 1;
        vector<int> ret;
        while(true) {
            if(i >= n || i < 0) {
                break;
            }
            if(j >= m || j < 0) {
                break;
            }
            if(visited[i][j]) {
                break;
            }
            ret.push_back(a[i][j]);
            visited[i][j] = true;
            if(j+dj >= m || (dj > 0 && visited[i+di][j+dj])) {
                // cant go right
                di = 1;
                dj = 0;
            }
            if(j+dj < 0 || (dj < 0 && visited[i+di][j+dj])) {
                // cant go left
                di = -1;
                dj = 0;
            }
            if(i+di >= n || (di > 0 && visited[i+di][j+dj])) {
                // cant go down
                di = 0;
                dj = -1;
            }
            if(i+di < 0 || (di < 0 && visited[i+di][j+dj])) {
                // cant go up
                di = 0;
                dj = 1;
            }
            i = i + di;
            j = j + dj;
        }
        return ret;
    }
};