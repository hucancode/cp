class Solution {
public:
    vector<vector<int>> findWinners(vector<vector<int>>& matches) {
        set<int> played;
        vector<bool> lose1(1e5, false);
        vector<bool> loseMany(1e5, false);
        for(auto match: matches) {
            played.insert(match[0]);
            played.insert(match[1]);
            if(loseMany[match[1]]) {
                continue;
            }
            if(lose1[match[1]]) {
                lose1[match[1]] = false;
                loseMany[match[1]] = true;
            } else {
                lose1[match[1]] = true;
            }
        }
        vector<vector<int>> ret(2);
        for(auto i: played) {
            if(loseMany[i]) {
                continue;
            }
            if(lose1[i]) {
                ret[1].push_back(i);
                continue;
            }
            ret[0].push_back(i);
        }
        return ret;
     }
};