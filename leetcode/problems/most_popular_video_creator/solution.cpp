class Solution {
public:
    vector<vector<string>> mostPopularCreator(vector<string>& creators, vector<string>& ids, vector<int>& views) {
        int n = creators.size();
        map<string, long long> creator_view;
        map<string, pair<int, string>> creator_top;
        for(int i = 0;i<n;i++) {
            auto c = creators[i];
            auto id = ids[i];
            auto v = views[i];
            creator_view[c] += v;
            if(creator_top.find(c) == creator_top.end()) {
                creator_top[c] = make_pair(v, id);
            } else {
                int top_v;
                string top;
                tie(top_v, top) = creator_top[c];
                if(v > top_v || (top_v == v && id.compare(top) < 0)) {
                    creator_top[c] = make_pair(v, id);
                }
            }
        }
        vector<vector<string>> ret;
        long long best = -1;
        for(auto kv: creator_view) {
            string k = kv.first;
            long long v = (long long)kv.second;
            if(v < best) {
                continue;
            }
            if(v > best) {
                ret.clear();
            }
            best = v;
            ret.push_back({k, creator_top[k].second});
        }
        return ret;
    }
};