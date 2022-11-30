class Solution {
public:
    long long appealSum(string s) {
        long long ret = 0;
        int n = s.size();
        int m = 'z'-'a'+1;
        vector<vector<int>> lastIdx(m, vector<int>(n, -1));
        for(int c = 0;c<m;c++) {
            for(int j = 0;j<n;j++) {
                if(s[j] - 'a' == c) {
                    lastIdx[c][j] = j;
                } else if(j > 0) {
                    lastIdx[c][j] = lastIdx[c][j-1];
                }
            }
        }
        vector<vector<long long>> f(m, vector<long long>(n, 0));
        // f[c][j] = total score of all substring end with j, that has c character in it
        for(int c = 0;c<m;c++) {
            for(int j = 0;j<n;j++) {
                if(lastIdx[c][j] == -1) {
                    continue;
                }
                f[c][j] = lastIdx[c][j]+1;// there are k + 1 way to make substring that contain c
                ret += f[c][j];
            }
        }
        return ret;
    }
};