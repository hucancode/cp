class Solution {
public:
    int numTeams(vector<int>& rating) {
        int n = rating.size();
        vector<int> leftSmall(n, 0);
        vector<int> leftBig(n, 0);
        vector<int> rightSmall(n, 0);
        vector<int> rightBig(n, 0);
        vector<int> a;
        a.reserve(n);
        a.push_back(rating[0]);
        for(int i = 1;i<n;i++) {
            auto it = lower_bound(a.begin(), a.end(), rating[i]);
            auto it2 = upper_bound(a.begin(), a.end(), rating[i]);
            leftSmall[i] = distance(a.begin(), it);
            leftBig[i] = distance(it2, a.end());
            a.insert(it, rating[i]);
        }
        a.resize(0);
        a.push_back(rating[n-1]);
        for(int i = n-2;i>=0;i--) {
            auto it = lower_bound(a.begin(), a.end(), rating[i]);
            auto it2 = upper_bound(a.begin(), a.end(), rating[i]);
            rightSmall[i] = distance(a.begin(), it);
            rightBig[i] = distance(it2, a.end());
            a.insert(it, rating[i]);
        }
        int ret = 0;
        for(int i = 1;i<n-1;i++) {
            ret += leftSmall[i]*rightBig[i];
            ret += leftBig[i]*rightSmall[i];
        }
        return ret;
    }
};