class Solution {
public:
    string kthSmallestPath(vector<int>& destination, int k) {
        int n = destination[1]+1;
        int m = destination[0]+1;
        vector<vector<int>> f(n, vector<int>(m,0));
        f[0][0] = 1;
        for(int i = 0;i<n;i++) {
            f[i][0] = 1;
        }
        for(int i = 0;i<m;i++) {
            f[0][i] = 1;
        }
        for(int i = 1;i<n;i++) {
            for(int j = 1;j<m;j++) {
                f[i][j] = f[i-1][j] + f[i][j-1];
            }
        }
        string ret = "";
        int i = 0;
        int j = 0;
        int rank = 1;
        while(i<n && j<m) {
            int dx = n-1-i;
            int dy = m-1-j;
            if(dx == 0 && dy == 0) {
                break;
            }
            if(dx == 0) {
                j++;
                ret += "V";
            } else if(dy == 0) {
                i++;
                ret += "H";
            } else {
                int deltaV = f[dx][dy] - f[dx][dy-1]; 
                // if we go down, we go down deltaV in rank
                // if we go right, our rank stay the same
                if(rank + deltaV <= k) {
                    rank += deltaV;
                    j++;
                    ret += "V";
                } else {
                    i++;
                    ret += "H";
                }
            }
        }
        return ret;
        cout<<"f: "<<endl;
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                cout<<f[i][j]<<' ';
            }
            cout<<endl;
        }
        cout<<endl;
        return ret;
    }
};