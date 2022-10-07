class Solution {
public:
    int partitionString(string s) {
        int n = s.size();
        int m = 'z'-'a'+1;
        vector<vector<int>> count(n+1, vector<int>(m, 0));
        for(int i = 1;i<=n;i++) {
            for(int j = 0;j<m;j++) {
                count[i][j] = count[i-1][j];
            }
            auto key = s[i-1] - 'a';
            count[i][key]++;
        }
        vector<int> f(n+1, 0);
        f[0] = 0;
        int i = 1;
        int j = 0;
        while(i<=n) {
            bool sjiUnique = true;
            for(int k = 0;k<m;k++) {
                if(count[i][k] - count[j][k] > 1) {
                    sjiUnique = false;
                    break;
                }
            }
            //cout<<"check i "<<i<<" j "<<j<<endl;
            //cout<<s.substr(j, i-j)<<" unique = "<<sjiUnique<<endl;
            if(!sjiUnique) {
                j++;
            } else  {
                f[i] = f[j]+1;
                //cout<<"f["<<i<<"] = f["<<j<<"]+1"<<"="<<f[i]<<endl;
                i++;
            }
        }
        return f[n];
    }
};