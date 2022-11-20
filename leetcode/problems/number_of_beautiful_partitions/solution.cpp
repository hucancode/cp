class Solution {
public:
    int INF = 1e9+7;
    int fact(int k) {
        if(k<=1) {
            return 1;
        }
        long long ret = 1;
        ret = ret * k * fact(k-1);
        ret %= INF;
        return ret;
    }
    bool isPrime(char c) {
        return c == '2' || c=='3' || c=='5' || c=='7';
    }
    int beautifulPartitions(string s, int k, int minLength) {
        if(minLength == 1) {
            minLength = 2;
        }
        int n = s.size();
        if(k == 1) {
            if(isPrime(s[0]) && !isPrime(s[n-1]) && n >= minLength) {
                return 1;
            } else {
                return 0;
            }
        }
        vector<int> pivot;
        int i0 = minLength;
        for(int i = i0;i<n-i0+1;i++) {
            if(!isPrime(s[i]) || (i>0 && isPrime(s[i-1]))) {
                continue;
            }
            pivot.push_back(i);
        }
        int m = pivot.size();
        if(m + 1 == k) {
            return 1;
        }
        int INF = 1e9+7;
        int ret = 0;
        vector<vector<int>> f(m, vector<int>(k, 0));
        // f[i][j] = number of way to use pivot[0~(i-1)] to make j parts
        for(int i = 0;i<m;i++) {
            f[i][1] = 1;
        }
        for(int i = 0;i<m;i++) {
            int a = pivot[i];
            for(int j = 1;j<k;j++) {
                for(int prev = i-1;prev>=0;prev--) {
                    int b = pivot[prev];
                    int d = a-b;
                    if(d < minLength) {
                        continue;
                    }
                    if(f[prev][j-1]==0) {
                        break;
                    }
                    f[i][j] += f[prev][j-1];
                    f[i][j] %= INF;
                }
            }
        }
        for(int i = 0;i<m;i++) {
            ret += f[i][k-1];
            ret %= INF;
        }
        return ret;
        cout<<"pivot("<<pivot.size()<<")"<<endl;
        for(int i = 0;i<m;i++) {
            cout<<pivot[i]<<" ";
        }
        cout<<endl;
        cout<<"f: "<<endl;
        for(int i = 0;i<m;i++) {
            cout<<"s["<<pivot[i]<<"]("<<s[pivot[i]]<<") ";
            for(int j = 1;j<k;j++) {
                cout<<f[i][j]<<" ";
            }
            cout<<endl;
        }
        return ret;
    }
};