class Solution {
public:
    int findRadius(vector<int>& houses, vector<int>& heaters) {
        sort(houses.begin(), houses.end());
        sort(heaters.begin(), heaters.end());
        int ret = 0;
        auto i = houses.begin();
        auto j = heaters.begin();
        while(i != houses.end()) {
            j = upper_bound(j, heaters.end(), *i);
            int d = 1e9;
            if(j != heaters.begin()) {
                d = min(d, abs(*i - *(j-1)));
            }
            if(j != heaters.end()) {
                d = min(d, abs(*i - *j));
            }
            ret = max(ret, d);
            i++;
        }
        return ret;
    }
};
