class Solution {
public:
    vector<int> fullBloomFlowers(vector<vector<int>>& flowers, vector<int>& persons) {
        int n = flowers.size();
        int m = persons.size();
        vector<int> open(n);
        vector<int> close(n);
        for(int i = 0;i<n;i++) {
            open[i] = close[i] = i;
        }
        auto openCmp = [&](const int& a, const int& b) {
            return flowers[a][0] < flowers[b][0];
        };
        auto closeCmp = [&](const int& a, const int& b) {
            return flowers[a][1] < flowers[b][1];
        };
        sort(open.begin(), open.end(), openCmp);
        sort(close.begin(), close.end(), closeCmp);
        vector<int> ret(m);
        for(int i = 0;i<m;i++) {
            auto openCmp = [&](const int& time, const int& b) {
                return time < flowers[b][0];
            };
            auto closeCmp = [&](const int& a, const int& time) {
                return flowers[a][1] < time;
            };
            auto openCount = distance(open.begin(), upper_bound(open.begin(), open.end(), persons[i], openCmp));
            auto closeCount = distance(close.begin(), lower_bound(close.begin(), close.end(), persons[i], closeCmp));
            ret[i] = openCount - closeCount;
        }
        return ret;
    }
}; 