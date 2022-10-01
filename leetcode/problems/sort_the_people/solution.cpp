class Solution {
public:
    vector<string> sortPeople(vector<string>& names, vector<int>& heights) {
        int n = names.size();
        vector<int> id(n);
        for(int i = 0;i<n;i++) {
            id[i] = i;
        }
        auto cmp = [&](const int a, const int b) {
            return heights[a] > heights[b];
        };
        sort(id.begin(), id.end(), cmp);
        vector<string> ret(n);
        for(int i = 0;i<n;i++) {
            ret[i] = names[id[i]];
        }
        return ret;
    }
};