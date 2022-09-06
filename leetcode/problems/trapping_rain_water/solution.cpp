class Solution {
public:
    int trap(vector<int>& a) {
        int n = a.size();
        vector<int> fl(n, 0);
        vector<int> fr(n, 0);
        fl[0] = a[0];
        for(int i = 1;i<n;i++) {
            fl[i] = max(fl[i-1], a[i]);
        }
        fr[n-1] = a[n-1];
        for(int i = n-2;i>=0;i--) {
            fr[i] = max(fr[i+1], a[i]);
        }
        int ret = 0;
        for(int i = 1;i<n-1;i++) {
            int h = min(fl[i], fr[i]);
            ret += h - a[i];
        }
        return ret;
    }
};