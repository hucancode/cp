class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        const int INF = 20001;
        vector<int> f(INF, 0);
        vector<int> ret;
        for(const auto& x: nums) {
            f[x+10000]++;
        }
        auto cmp = [&](const int a, const int b) {
            return f[a] > f[b];
        };
        for(int i = 0;i<INF;i++) {
            if(f[i] == 0) {
                continue;
            }
            ret.insert(upper_bound(ret.begin(), ret.end(), i, cmp), i);
            if(ret.size() > k) {
                ret.pop_back();
            }
        }
        for(auto& x: ret) {
            x -= 10000;
        }
        return ret;
    }
};