class Solution {
public:
    long long putMarbles(vector<int>& weights, int k) {
        int n = weights.size();
        typedef long long ll;
        vector<ll> f;
        for(int i = 1;i<n;i++) {
            f.push_back(weights[i] + weights[i-1]);
        }
        sort(f.begin(), f.end());
        ll ret=0;
        for(int i = 0;i<k-1;i++) {
            ret += f[n-i-2] - f[i];
        }
        return ret;
    }
};