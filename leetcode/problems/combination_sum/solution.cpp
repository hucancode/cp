class Solution {
public:
    vector<vector<int>> calculate(vector<int>::iterator begin, vector<int>::iterator end, int target) {
        vector<vector<int>> ret;
        if(target == 0) {
            ret.resize(1);
            return ret;
        }
        if(begin == end || target < 0) {
            return ret;
        }
        int sum = 0;
        vector<int> left;
        while(sum <= target) {
            auto rights = calculate(begin + 1, end, target - sum);
            for(const auto& right: rights) {
                vector<int> both = left;
                both.insert(both.end(), right.begin(), right.end());
                ret.push_back(both);
            }
            auto v = *begin;
            left.push_back(v);
            sum += v;
        }
        return ret;
    }
    vector<vector<int>> combinationSum(vector<int>& a, int target) {
        sort(a.begin(), a.end());
        int n = distance(a.begin(), upper_bound(a.begin(), a.end(), target));
        a.resize(n);
        set<vector<int>> ret;
        for(auto it = a.begin();it != a.end();it++) {
            auto items = calculate(it, a.end(), target);
            ret.insert(items.begin(), items.end());
        }
        return vector<vector<int>>(ret.begin(), ret.end());
    }
};