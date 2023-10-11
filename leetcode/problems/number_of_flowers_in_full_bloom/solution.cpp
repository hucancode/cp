class Solution {
public:
    vector<int> fullBloomFlowers(vector<vector<int>>& flowers, vector<int>& persons) {
        int n = flowers.size();
        int m = persons.size();
        vector<int> bloom(n);
        vector<int> wither(n);
        for(int i = 0;i<n;i++) {
            bloom[i] = flowers[i][0];
            wither[i] = flowers[i][1];
        }
        sort(bloom.begin(), bloom.end());
        sort(wither.begin(), wither.end());
        vector<int> ret(m);
        vector<pair<int,int>> queries;
        queries.reserve(m);
        for(int i = 0;i<m;i++) {
            queries.push_back(make_pair(persons[i],i));
        }
        sort(queries.begin(), queries.end());
        auto lastWithered = wither.begin();
        for(auto q: queries) {
            int time, i;
            tie(time, i) = q;
            lastWithered = lower_bound(lastWithered, wither.end(), time);
            auto witheredCount = distance(wither.begin(), lastWithered);
            auto lastBloomed = upper_bound(bloom.begin()+witheredCount, bloom.end(), time);
            auto bloomedCount = distance(bloom.begin(), lastBloomed);
            auto blooming = bloomedCount - witheredCount;
            ret[i] = blooming;
        }
        return ret;
    }
}; 