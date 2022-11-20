class Solution {
public:
    int maxSatisfaction(vector<int>& weight) {
        sort(weight.begin(), weight.end());
        int n = weight.size();
        vector<vector<int>> f(n+1, vector<int>(n+1, -2e9));
        // f[i][j] = use disc i to fill index j, what is the max satisfaction
        int ret = 0;
        for(int i = 0;i<=n;i++) {
            f[i][0] = 0;
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=i;j++) {
                int cost = weight[i-1] * j;
                int served = f[i-1][j-1] + cost;
                int passed = f[i-1][j];
                f[i][j] = max(served, passed);
                ret = max(ret, f[i][j]);
            }
        }
        return ret;
        cout<<"w : ";
        for(int i = 0;i<n;i++) {
            cout<<weight[i]<<" ";
        }
        cout<<endl;
        cout<<"f: "<<endl;
        for(int i = 0;i<=n;i++) {
            if(i == 0) {
                cout<<"w_ ";
            } else {
                cout<<"w"<<weight[i-1]<<" ";
            }
            for(int j = 0;j<=n;j++) {
                cout<<f[i][j]<<" ";
            }
            cout<<endl;
        }
        return ret;
    }
};