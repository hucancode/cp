class Solution {
public:
    bool overlap(vector<int>& a, vector<int>& b) {
        int l = max(a[0], b[0]);
        int r = min(a[1], b[1]);
        return l<=r;
    }
    void merge(vector<int>& a, vector<int>& b) {
        a[0] = min(a[0],b[0]);
        a[1] = max(a[1],b[1]);
    }
    vector<vector<int>> merge(vector<vector<int>>& a) {
        sort(a.begin(), a.end());
        auto l = a.begin();
        auto r = l+1;
        while(r!= a.end()) {
            if(overlap(*l, *r)) {
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