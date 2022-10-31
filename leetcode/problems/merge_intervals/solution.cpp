class Solution {
public:
    bool overlap(int l1, int r1, int l2, int r2) {
        int l = max(l1, l2);
        int r = min(r1, r2);
        return l<=r;
    }
    void merge(vector<int>& a, vector<int>& b) {
        a[0] = min(a[0],b[0]);
        a[1] = max(a[1],b[1]);
    }
    vector<vector<int>> merge(vector<vector<int>>& a) {
        sort(a.begin(), a.end(), 
            [](const vector<int>& a, const vector<int>& b) { 
                return a[0] < b[0];
            });
        vector<vector<int>> ret;
        ret.reserve(a.size());
        auto l = a.begin();
        auto r = l+1;
        while(r!= a.end()) {
            if(overlap((*l)[0], (*l)[1], (*r)[0], (*r)[1])) {
                merge(*l, *r);
            } else {
                r = a.erase(l+1,r);
                l = r;
            }
            r++;
        }
        a.erase(l+1,r);
        return a;
    }
};