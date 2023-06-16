typedef long long ll;
#define INF 1000000007
class Solution {
    vector<vector<ll>> c;
public:
    void build(int n) {
        // c[n][k] = the number of way to select k item from pool size n
        c.resize(n+1);
        for(int i=0;i<=n;i++) {
            c[i].resize(i+1);
            c[i][0] = c[i][i] = 1;
            for(int j=1;j<i;j++) {
                c[i][j] = (c[i-1][j-1] + c[i-1][j])%INF;
            }
        }
    }
    ll query(vector<int>& nums) {
        int n = nums.size();
        if(n <= 2) {
            return 1;
        }
        int p = nums[0];
        vector<int> left;
        vector<int> right;
        for(int i = 1;i<n;i++) {
            if(nums[i] < p) left.push_back(nums[i]);
            else right.push_back(nums[i]);
        }
        ll ret = c[n-1][left.size()];
        ret *= query(left);
        ret %= INF;
        ret *= query(right);
        ret %= INF;
        return ret;
    }
    int numOfWays(vector<int>& nums) {
        build(nums.size());
        return query(nums) - 1;
    }
};