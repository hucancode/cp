class Solution {
public:
    long long findScore(vector<int>& nums) {
        typedef pair<int, int> state;
        priority_queue<state, vector<state>, greater<state>> q;
        int n = nums.size();
        vector<bool> vis(n, false);
        for(int i = 0;i<n;i++) q.emplace(nums[i], i);
        long long ret = 0;
        while(!q.empty()) {
            int x, i;
            tie(x, i) = q.top();
            q.pop();
            if(vis[i]) continue;
            ret += x;
            vis[i] = true;
            if(i > 0) vis[i-1] = true;
            if(i < n-1) vis[i+1] = true;
        }
        return ret;
    }
};