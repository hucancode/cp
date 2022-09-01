class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& a, vector<int>& b) {
        if(a.empty()) {
            a.push_back(b);
            return a;
        }
        int n = a.size();
        int l = n;
        int r = -1;
        for(int i = 0;i<n;i++) {
            if(b[0] > a[i][1]) {
                continue;
            }
            if(b[1] < a[i][0]) {
                l = min(i, l);
                break;
            }
            b[0] = min(b[0], a[i][0]);
            b[1] = max(b[1], a[i][1]);
            l = min(i, l);
            r = max(i, r);
        }
        if(l <= r) {
            a.erase(a.begin() + l, a.begin() + r + 1);
        }
        a.insert(a.begin() + l, b);
        return a;
    }
};