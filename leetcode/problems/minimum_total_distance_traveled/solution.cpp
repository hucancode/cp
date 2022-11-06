class Solution {
public:
    long long minimumTotalDistance(vector<int>& robot, vector<vector<int>>& factory) {
        int n = robot.size();
        int m = factory.size();
        sort(robot.begin(), robot.end());
        sort(factory.begin(), factory.end());
        typedef long long ll;
        ll ret;
        vector<vector<ll>> f(m+1, vector<ll>(n+1,1e18));
        // f[i][j] = min cost use factory [0,i] to handle robot [0,j]
        for(int i = 0;i<=m;i++) {
            f[i][0] = 0;
        }
        for(int i = 1;i<=m;i++) {
            int pos = factory[i-1][0];
            int cap = factory[i-1][1];
            for(int j = 1;j<=n;j++) {
                long long cost = 0;
                for(int k = 0;k<=cap;k++) {
                    if(j-k < 0) {
                        continue;
                    }
                    long long prev = f[i-1][j-k];
                    f[i][j] = min(f[i][j], cost + prev);
                    if(j-k < 1) {
                        continue;
                    }
                    cost += abs(robot[j-1-k] - pos);
                }
            }
        }
        // cout<<"f = "<<endl;
        // for(auto x: f) {
        //     for(auto y: x) {
        //         cout<<y<<" ";
        //     }
        //     cout<<endl;
        // }
        return f[m][n];
    }
};