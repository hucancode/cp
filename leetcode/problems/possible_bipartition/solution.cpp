class Solution {
public:
    bool possibleBipartition(int n, vector<vector<int>>& dislikes) {
        const int NO_GROUP = 0;
        const int GROUP_1 = 1;
        const int GROUP_2 = 2;
        vector<int> group(n+1, NO_GROUP);
        vector<vector<int>> hateMap(n+1);
        for(auto d: dislikes) {
            hateMap[d[0]].push_back(d[1]);
            hateMap[d[1]].push_back(d[0]);
        }
        for(auto d: dislikes) {
            queue<pair<int,int>> q;
            auto x = d[0];
            if(group[x] != NO_GROUP) {
                continue;
            }
            q.emplace(x, GROUP_1);
            while(!q.empty()) {
                int x, g;
                tie(x, g) = q.front();
                bool conflict = group[x] != NO_GROUP && group[x] != g;
                if(conflict) {
                    return false;
                }
                q.pop();
                if(group[x] == g) {
                    continue;
                }
                group[x] = g;
                auto next = (g == GROUP_1)?GROUP_2: GROUP_1;
                for(auto y: hateMap[x]) {
                    q.emplace(y, next);
                }
            }
        }
        return true;
    }
};