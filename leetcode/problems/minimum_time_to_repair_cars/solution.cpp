#define INF (100LL * 1000000LL * 1000000LL)
class Solution {
public:
    bool check(vector<int>& ranks, long long t, int n) {
        for(auto r: ranks) {
            int repaired = sqrt(t/r);
            n -= repaired;
            if(n <= 0) return true;
        }
        return false;
    }
    long long repairCars(vector<int>& ranks, int n) {
        sort(ranks.begin(), ranks.end());
        long long k = ranks[0];
        auto l = 0LL, r = k*n*n;
        while(l < r) {
            auto m = (l+r)/2;
            bool good = check(ranks, m, n);
            if(good) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
};