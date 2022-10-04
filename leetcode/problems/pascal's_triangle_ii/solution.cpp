class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> f(rowIndex+1, 0);
        int n = 0;
        while(n <= rowIndex) {
            int k = 1;
            for(int i = 0;i<=n;i++) {
                if(i == 0 || i == n) {
                    f[i] = 1;
                    continue;
                }
                auto nextk = f[i];
                f[i] += k;
                k = nextk;
            }
            n++;
        }
        return f;
    }
};