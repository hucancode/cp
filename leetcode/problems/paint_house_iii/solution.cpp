class Solution {
public:
    int minCost(vector<int>& houses, vector<vector<int>>& cost, int m, int n, int target) {
        const int INF = 2e6;
        typedef vector<int> vi;
        typedef vector<vi> vvi;
        typedef vector<vvi> v3i;
        // f[i][j][k] = at house i, paint color j, maintaining k neighborhood, 
        // what is the optimal cost
        v3i f(m, vvi(n, vi(target+1, INF)));
        if(houses[0] == 0) {
            for(int j = 0;j<n;j++) {
                int costToPaint = cost[0][j];
                f[0][j][1] = costToPaint;
            }
        } else {
            int j = houses[0]-1;
            f[0][j][1] = 0;
        }
        for(int i = 1;i<m;i++) {
            for(int j = 0;j<n;j++) {
                if(houses[i] != 0 && houses[i] != j+1) {
                    continue;
                }
                int costToPaint = (houses[i] != 0)?0:cost[i][j];
                for(int k = 1;k<=target;k++) {
                    int prevSameColor = f[i-1][j][k];
                    int prevNewColor = INF;
                    for(int j2 = 0;j2<n;j2++) {
                        if(j2 == j) continue;
                        prevNewColor = min(prevNewColor, f[i-1][j2][k-1]);
                    }
                    f[i][j][k] = min(f[i][j][k], prevNewColor + costToPaint);
                    f[i][j][k] = min(f[i][j][k], prevSameColor + costToPaint);
                }
            }
        }
        int ret = f[m-1][0][target];
        for(int j = 1;j<n;j++) {
            ret = min(ret, f[m-1][j][target]);
        }
        if(ret == INF) {
            ret = -1;
        }
        // for(int k = 1;k<=target;k++) {
        //     cout<<"target "<<k<<" neighbors"<<endl;
        //     for(int i = 0;i<m;i++) {
        //         cout<<"house "<<i<<": ";
        //         for(auto y: f[i]) {
        //             cout<<y[k]<<' ';
        //         }
        //         cout<<endl;
        //     }
        // }
        return ret;
    }
};