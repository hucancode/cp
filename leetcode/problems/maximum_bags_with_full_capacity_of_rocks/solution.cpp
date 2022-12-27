class Solution {
public:
    int maximumBags(vector<int>& capacity, vector<int>& rocks, int additionalRocks) {
        int n = rocks.size();
        vector<int> delta(n);
        for(int i = 0;i<n;i++) {
            delta[i] = capacity[i] - rocks[i];
        }
        sort(begin(delta), end(delta));
        int i = 0;
        while(i < n && additionalRocks > 0 && additionalRocks >= delta[i]) {
            additionalRocks -= delta[i];
            i++;
        }
        return i;
    }
};