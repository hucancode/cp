class Solution {
public:
    int maxTotalFruits(vector<vector<int>>& fruits, int startPos, int k) {
        int ret = 0;
        int pivot, l, r;
        vector<int> target = {startPos, 0};
        auto it = lower_bound(fruits.begin(), fruits.end(), target);
        pivot = distance(fruits.begin(), it);
        bool exist = it != fruits.end() && (*it)[0] == startPos;
        if(!exist) {
            fruits.insert(it, target);
        }
        int n = fruits.size();
        for(auto x: fruits) {
            cout<<x[0]<<" ";
        }
        cout<<endl;
        vector<long long> prefix(n+1, 0);
        for(int i = 1;i<=n;i++) {
            prefix[i] = prefix[i-1]+fruits[i-1][1];
        }
        vector<int> positions(n);
        for(int i = 0;i<n;i++) {
            positions[i] = fruits[i][0];
        }
        l = distance(positions.begin(), lower_bound(positions.begin(), positions.end(), startPos - k));
        for(;l<=pivot;l++) {
            int cost = positions[pivot] - positions[l];
            int next = positions[l] + (k - cost);
            int r = distance(positions.begin(), upper_bound(positions.begin(), positions.end(), next)) - 1;
            r = max(r, pivot);
            int score = prefix[r+1] - prefix[l];
            ret = max(ret, score);
        }
        r = distance(positions.begin(), upper_bound(positions.begin(), positions.end(), startPos + k)) - 1;
        for(;r>=pivot;r--) {
            int cost = positions[r] - positions[pivot];
            int next = positions[r] - (k - cost);
            int l = distance(positions.begin(), lower_bound(positions.begin(), positions.end(), next));
            l = min(l, pivot);
            int score = prefix[r+1] - prefix[l];
            ret = max(ret, score);
        }
        return ret;
    }
};