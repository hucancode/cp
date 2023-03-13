class Solution {
public:
    int maxScore(vector<int>& nums) {
        vector<int> positive;
        vector<int> negative;
        int zero = 0;
        for(auto x: nums) {
            if(x > 0) positive.push_back(x);
            if(x < 0) negative.push_back(x);
            if(x == 0) zero++;
        }
        if(positive.empty()) {
            return 0;
        }
        int ret = positive.size() + zero;
        long long p = accumulate(positive.begin(), positive.end(), 0LL);
        sort(negative.rbegin(), negative.rend());
        for(auto x: negative) {
            p += x;
            if(p > 0) ret++;
            else break;
        }
        return ret;
    }
};
