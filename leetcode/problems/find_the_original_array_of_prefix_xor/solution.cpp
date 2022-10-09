class Solution {
public:
    vector<int> findArray(vector<int>& pref) {
        int n = pref.size();
        vector<int> f(n, 0);
        f[0] = pref[0];
        int k = f[0];
        for(int i = 1;i<n;i++) {
            f[i] = k ^ pref[i];
            k = k ^ f[i];
        }
        return f;
    }
};