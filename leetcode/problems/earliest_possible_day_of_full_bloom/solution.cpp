typedef pair<int, int> pi;
class Solution {
public:
    int earliestFullBloom(vector<int>& plantTime, vector<int>& growTime) {
        int n = plantTime.size();
        vector<pi> f(n);
        for(int i = 0;i<n;i++) {
            f[i].first = growTime[i];
            f[i].second = plantTime[i];
        }
        sort(f.begin(), f.end());
        int sumPlant = 0;
        int bestGrow = 0;
        for(auto x:f) {
            bestGrow = max(bestGrow, x.first-sumPlant);
            sumPlant += x.second;
        }
        return sumPlant+bestGrow;
    }
};