class Solution {
public:
    bool uniqueOccurrences(vector<int>& arr) {
        vector<int> count(2001, 0);
        for(auto x: arr) {
            int key = x+1000;
            count[key]++;
        }
        sort(count.begin(), count.end());
        auto i = upper_bound(count.begin(), count.end(), 0);
        while(i != count.end()) {
            auto j = i + 1;
            if(j != count.end()) {
                if(*i == *j) {
                    return false;
                }
            }
            i++;
        }
        return true;
    }
};