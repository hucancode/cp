class Solution {
public:
    int numberOfWeakCharacters(vector<vector<int>>& chars) {
        int n = chars.size();
        // sort by atk
        sort(chars.begin(), chars.end(), [](const vector<int>& a, const vector<int>& b) -> bool
        {
            return a[0] > b[0];
        });
        // record max def until this i point
        vector<int> atk(n);
        vector<int> defMax(n);
        atk[0] = chars[0][0];
        defMax[0] = chars[0][1];
        for(int i = 1;i<n;i++) {
            atk[i] = chars[i][0];
            defMax[i] = max(defMax[i-1], chars[i][1]);
        }
        int ret = 0;
        for(const auto& x: chars) {
            auto it = upper_bound(atk.rbegin(), atk.rend(), x[0]);
            if(it == atk.rend()) {
                continue;
            }
            auto it2 = defMax.rbegin() + distance(atk.rbegin(), it);
            if(*it2 > x[1]) {
                ret++;
            }
        }
        return ret;
    }
};