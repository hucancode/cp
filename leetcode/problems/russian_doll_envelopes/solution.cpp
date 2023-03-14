class Solution {
public:
    int maxEnvelopes(vector<vector<int>>& envelopes) {
        sort(envelopes.begin(), envelopes.end(), [](const vector<int>& a, const vector<int>& b) {
            if(a[0] < b[0]) {
                return true;
            }
            if(a[0] > b[0]) {
                return false;
            }
            return a[1] > b[1];
        });
        vector<int> dolls;
        for(auto& a: envelopes) {
            auto h = a[1];
            auto it = lower_bound(dolls.begin(), dolls.end(), h);
            if(it == dolls.end()) {
                dolls.push_back(h);
            } else {
                *it = h;
            }
        }
        return dolls.size();
    }
};