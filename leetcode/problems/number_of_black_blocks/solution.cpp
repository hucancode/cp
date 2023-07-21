class Solution {
public:
    vector<long long> countBlackBlocks(int m, int n, vector<vector<int>>& coordinates) {
        vector<long long> ret(5,0);
        map<pair<int,int>, long long> mp;
        for(auto pos: coordinates) {
            int x = pos[0];
            int y = pos[1];
            for(int dx = 0;dx<=1;dx++) {
                for(int dy = 0;dy<=1;dy++) {
                    mp[{x+dx,y+dy}]++;
                }
            }
        }
        //cout<<"___________"<<endl;
        ret[0] = ((long long)m-1)*((long long)n-1);
        for(auto kv: mp) {
            auto k = kv.first;
            int v = kv.second;
            if(k.first == 0 || k.second == 0 || k.first >= m || k.second >= n) {
                continue;
            }
            //cout<<"pivot end at "<<k.first<<"-"<<k.second<<" has "<<v<<" blocks"<<endl;
            ret[v] += 1;
            ret[0]--;
        }
        return ret;
    }
};