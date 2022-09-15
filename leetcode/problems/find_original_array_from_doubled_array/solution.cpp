class Solution {
public:
    vector<int> findOriginalArray(vector<int>& changed) {
        vector<int> ret;
        if(changed.size() < 2 || changed.size()%2 == 1) {
            return ret;
        }
        map<int, int> counts;
        for(const auto& x: changed) {
            if(counts.find(x) == counts.end()) {
                counts[x] = 0;
            }
            counts[x]++;
        }
        ret.reserve(changed.size()/2);
        for(auto& item: counts) {
            int u = item.first;
            if(item.second == 0) {
                continue;
            }
            int v = u*2;
            if(counts.find(v) == counts.end()) {
                ret.clear();
                return ret;
            }
            if(counts[v] < item.second) {
                ret.clear();
                return ret;
            }
            if(u == v) {
                item.second /= 2;
            }
            ret.insert(ret.end(), item.second, u);
            counts[v] -= item.second;
            item.second = 0;
        }
        for(auto& item: counts) {
            if(item.second != 0) {
                ret.clear();
                return ret;
            }
        }
        return ret;
    }
};