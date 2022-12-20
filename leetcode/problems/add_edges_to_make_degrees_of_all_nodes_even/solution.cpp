class Solution {
public:
    bool isPossible(int n, vector<vector<int>>& edges) {
        map<int,int> degree;
        map<pair<int,int>, bool> connected;
        for(auto e: edges) {
            degree[e[0]]++;
            degree[e[1]]++;
            connected[{e[0],e[1]}] = true;
            connected[{e[1],e[0]}] = true;
        }
        vector<int> candidates;
        int usedEdge = 0;
        for(auto item: degree) {
            if(item.second % 2) {
                candidates.push_back(item.first);
            }
        }
        if(candidates.size() == 0) {
            return true;
        }
        if(candidates.size() == 2) {
            int a = candidates[0];
            int b = candidates[1];
            if(!connected[{a,b}]) {
                return true;
            }
            for(int c = 1;c<=n;c++) {
                if(!connected[{a,c}] && !connected[{b,c}]) {
                    return true;
                }
            }
            return false;
        }
        if(candidates.size() == 3) {
            int a = candidates[0];
            int b = candidates[1];
            int c = candidates[2];
            if((!connected[{a,c}] && !connected[{b,c}]) || 
            (!connected[{a,b}] && !connected[{b,c}]) || 
            (!connected[{a,b}] && !connected[{a,c}])) {
                return true;
            }
            return false;
        }
        if(candidates.size() == 4) {
            int a = candidates[0];
            int b = candidates[1];
            int c = candidates[2];
            int d = candidates[3];
            if((!connected[{a,b}] && !connected[{c,d}]) || 
            (!connected[{a,d}] && !connected[{b,c}]) || 
            (!connected[{a,c}] && !connected[{b,d}])) {
                return true;
            }
            return false;
        }
        return false;
    }
};