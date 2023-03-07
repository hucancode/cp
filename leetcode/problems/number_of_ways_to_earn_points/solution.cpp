class Solution {
public:
    int waysToReachTarget(int target, vector<vector<int>>& types) {
        const int INF = 1e9+7;
        vector<long long> f(target+1, 0);
        f[0] = 1;
        for(auto& t: types) {
            int count = t[0];
            int mark = t[1];
            for(int j = target;j>=0;j--) {
                for(int k = count*mark;k>=mark;k-=mark) {
                    int next = k+j;
                    if(next > target) continue;
                    f[next] += f[j];
                    f[next] %= INF;
                }
            }
        }
        return f[target];
    }
};