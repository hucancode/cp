class Solution {
public:
    vector<int> findBall(vector<vector<int>>& a) {
        int n = a.size();
        int m = a[0].size();
        vector<int> b(m);
        for(int i = 0;i<m;i++) {
            b[i] = i;
        }
        for(int i = 0;i<n;i++) { 
            //cout<<"row "<<i<<endl;
            vector<int> c(b.begin(), b.end());
            for(int j = 0;j<m;j++) {
                bool openLeft = false;
                if(j > 0) {
                    openLeft = a[i][j] == 1 && a[i][j-1] == 1;
                }
                bool openRight = false;
                if(j < m-1) {
                    openRight = a[i][j] == -1 && a[i][j+1] == -1;
                }
                if(openLeft) {
                    c[j] = b[j-1];
                } else if(openRight) {
                    c[j] = b[j+1];
                } else {
                    c[j] = -1;
                }
                //cout<<"open "<<openLeft<<"-"<<openRight<<" b["<<j<<"]="<<c[j]<<endl;
            }
            b = c;
        }
        vector<int> ret(m, -1);
        for(int i = 0;i<m;i++) {
            if(b[i] == -1) {
                continue;
            }
            ret[b[i]] = i;
        }
        return ret;
    }
};