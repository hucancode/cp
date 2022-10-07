class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        int n = matrix.size();
        int m = matrix[0].size();
        int ret = 0;
        vector<vector<int>> row1(n, vector<int>(m, 0));
        vector<vector<int>> col1(n, vector<int>(m, 0));
        vector<vector<int>> f(n, vector<int>(m, 0));
        for(int i = 0;i<n;i++) {
            row1[i][0] = matrix[i][0] == '1';
            for(int j = 1;j<m;j++) {
                if(matrix[i][j] == '0') {
                    row1[i][j] = 0;
                } else {
                    row1[i][j] = row1[i][j-1]+1;
                }
            }
        }
        for(int j = 0;j<m;j++) {
            col1[0][j] = matrix[0][j] == '1';
            for(int i = 1;i<n;i++) {
                if(matrix[i][j] == '0') {
                    col1[i][j] = 0;
                } else {
                    col1[i][j] = col1[i-1][j]+1;
                }
            }
        }
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                auto prev = (i>0 && j>0)?f[i-1][j-1]:0;
                auto d = min(prev+1,min(row1[i][j], col1[i][j]));
                f[i][j] = d;
                ret = max(ret, f[i][j]);
            }
        }
        cout<<"row: "<<endl;
        // for(int i = 0;i<n;i++) {
        //     for(int j = 0;j<m;j++) {
        //         cout<<row1[i][j]<<" ";
        //     }
        //     cout<<endl;
        // }
        // cout<<"col: "<<endl;
        // for(int i = 0;i<n;i++) {
        //     for(int j = 0;j<m;j++) {
        //         cout<<col1[i][j]<<" ";
        //     }
        //     cout<<endl;
        // }
        return ret*ret;
    }
};