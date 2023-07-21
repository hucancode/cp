class Solution {
public:
    vector<int> relocateMarbles(vector<int>& nums, vector<int>& moveFrom, vector<int>& moveTo) {
        map<int, bool> occupied;
        for(auto x: nums) occupied[x] = true;
        for(int i = 0;i<moveFrom.size();i++) {
            if(!occupied[moveFrom[i]]) continue;
            occupied[moveFrom[i]] = false;
            occupied[moveTo[i]] = true;
        }
        vector<int> ret;
        for(auto kv: occupied) if(kv.second) ret.push_back(kv.first);
        return ret;
    }
};