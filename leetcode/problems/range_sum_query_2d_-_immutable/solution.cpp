class NumMatrix {
public:
    vector<vector<int>> data;
    NumMatrix(vector<vector<int>>& matrix) {
        int n = matrix.size();
        int m = matrix[0].size();
        data.resize(n+1, vector<int>(m+1, 0));
        for(int i = 1;i<=n;i++) {
            int rowSum = 0;
            for(int j = 1;j<=m;j++) {
                rowSum += matrix[i-1][j-1];
                data[i][j] = rowSum + data[i-1][j];
            }
        }
    }
    
    int sumRegion(int row1, int col1, int row2, int col2) {
        row2++;
        col2++;
        auto big = data[row2][col2];
        auto small = data[row1][col1];
        auto left = data[row2][col1];
        auto up = data[row1][col2];
        return big - left - up + small;
    }
};

/**
 * Your NumMatrix object will be instantiated and called as such:
 * NumMatrix* obj = new NumMatrix(matrix);
 * int param_1 = obj->sumRegion(row1,col1,row2,col2);
 */