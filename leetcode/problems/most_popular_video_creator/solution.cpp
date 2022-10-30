class Solution {
public:
    vector<vector<string>> mostPopularCreator(vector<string>& creators, vector<string>& ids, vector<int>& views) {
        vector<vector<string>> ret;
        map<string, long long> totalView;
        map<string, int> topVideo;
        int n = creators.size();
        for(int i = 0;i<n;i++) {
            auto& c = creators[i];
            auto& id = ids[i];
            auto& view = views[i];
            totalView[c] += view;
            if(topVideo.find(c) == topVideo.end()) {
                topVideo[c] = i;
            }
            auto topId = topVideo[c];
            if(views[topId] < view || (views[topId] == view && ids[topId] > id)) {
                topVideo[c] = i;
            }
        }
        vector<pair<long long, string>> f;
        for(auto& item: totalView) {
            f.emplace_back(item.second, item.first);
        }
        sort(f.rbegin(), f.rend());
        long long k = f[0].first;
        for(auto& item: f) {
            if(item.first != k) {
                break;
            }
            auto creator = item.second;
            auto top = ids[topVideo[creator]];
            ret.push_back({creator, top});
        }
        return ret;
    }
};