class Solution {
public:
    long long minCost(vector<int>& nums, vector<int>& cost) {
        int n = nums.size();
        vector<pair<int, int>> a;
        a.reserve(n);
        for(int i = 0;i<n;i++) {
            a.emplace_back(nums[i], cost[i]);
        }
        sort(a.begin(), a.end());
        vector<long long> f(n);
        f[0] = 0;
        long long c = a[0].second;
        for(int i = 1;i<n;i++) {
            int d = a[i].first - a[i-1].first;
            f[i] = f[i-1] + c * d;
            c += a[i].second;
            //cout<<"cost to make left equal "<<a[i].first<<" = "<<f[i]<<endl;
        }
        vector<long long> g(n);
        c = a[n-1].second;
        g[n-1] = 0;
        for(int i = n-2;i>=0;i--) {
            int d = a[i+1].first - a[i].first;
            g[i] = g[i+1] + c * d;
            c += a[i].second;
            //cout<<"cost to make right equal "<<a[i].first<<" = "<<g[i]<<endl;
        }
        
        long long ret = 1e18;
        for(int i = 0;i<n;i++) {
            //cout<<"cost to make left equal "<<a[i].first<<" = "<<f[i]<<endl;
            //cout<<"cost to make right equal "<<a[i].first<<" = "<<g[i]<<endl;
            ret = min(ret, f[i]+g[i]);
        }
        return ret;
    }
};