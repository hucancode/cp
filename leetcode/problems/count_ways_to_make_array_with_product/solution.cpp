class Solution {
public:
    static const int INF = 1e9+7;
    vector<bool> prime;
    vector<vector<int>> ncr;
    bool initialized = false;
    void build() {
        if(initialized) {
            return;
        }
        prime.resize(1e4+1, true);
        prime[0] = false;
        prime[1] = false;
        for(int i = 2;i<=1e4;i++) {
            if(!prime[i]) {
                continue;
            }
            for(int j = 2;i*j<=1e4;j++) {
                prime[i*j] = false;
            }
        }
        // repeat count of prime factor of n never go pass 13, because smallest prime is 2 and 2^14 > 1e4
        // so we only need to precalculate until 13
        // nCr precalculation formular based on Pascal's Triangle, idea learnt from votrubac
        // seemore: https://leetcode.com/problems/count-ways-to-make-array-with-product/discuss/1035607/C%2B%2BPython-Precompute
        ncr.resize(1e4+13, vector<int>(14,0));
        ncr[0][0] = 1;
        for (int n = 1; n<1e4+13; n++) {
            ncr[n][0] = 1;
            for (int r = 1;r<14; r++) {
                ncr[n][r] = (ncr[n-1][r-1] + ncr[n-1][r]) % INF;
            }
        }
        initialized = true;
    }
    vector<pair<int,int>> primeFactor(int n) {
        vector<pair<int,int>> ret;
        int k = 2;
        while(k <= n) {
            if(prime[k]) {
                int count = 0;
                while(n%k == 0) {
                    n = n/k;
                    count++;
                }
                if(count > 0) {
                    ret.emplace_back(k, count);
                }
            }
            k++;
        }
        return ret;
    }
    vector<int> waysToFillArray(vector<vector<int>>& queries) {
        vector<int> ret;
        build();
        for(auto q: queries) {
            long long ans = 1;
            int n = q[0];
            int k = q[1];
            auto factors = primeFactor(k);
            for(auto x: factors) {
                int count = x.second;
                ans = ans * ncr[count+n-1][count];
                ans %= INF;
            }
            ret.push_back(ans);
        }
        return ret;
    }
};