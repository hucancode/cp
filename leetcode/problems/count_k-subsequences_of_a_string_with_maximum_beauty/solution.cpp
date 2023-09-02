class Solution {
public:
    long long ncr(int n, int r) {
        //cout<<"ncr "<<n<<","<<r<<endl;
        long long ret = 1;
        for(int i = r+1;i<=n;i++) {
            ret *= i;
        }
        for(int i = 2;i<=n-r;i++) {
            ret /= i;
        }
        //cout<<"return "<<ret<<endl;
        return ret;
    }
    int countKSubsequencesWithMaxBeauty(string s, int k) {
        const int INF = 1e9+7;
        map<char, int> count;
        for(auto c:s) {
            count[c]++;
        }
        if(count.size() < k) {
            return 0;
        }
        vector<int> pool;
        for(auto kv: count) {
            pool.push_back(kv.second);
        }
        sort(pool.rbegin(), pool.rend());
        int r = k;
        int n = k;
        while(n < pool.size() && pool[n-1] == pool[n]) {
            n++;
        }
        long long ret = 1;
        for(int i = 0;i<r;i++) {
            ret *= pool[i];
            ret %= INF;
        }
        int tail_count = 0;
        for(int i = n-1;i>=0;i--) {
            if(pool[i] != pool[r-1]) {
                break;
            }
            tail_count++;
        }
        int tail_needed = r-(n-tail_count);
        ret *= ncr(tail_count, tail_needed);
        ret %= INF;
        return ret;
    }
};