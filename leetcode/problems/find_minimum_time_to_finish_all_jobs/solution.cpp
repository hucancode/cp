class Solution {
public:
    int minimumTimeRequired(vector<int>& jobs, int k) {
        // AC but still bad implementation, TLE with this test
        // [10001,10002,10003,10004,10005,10006,10007,10008,10009,10010,10011,1000000]
        // 10
        int n = jobs.size();
        if(k == n) {
            return *max_element(jobs.begin(), jobs.end());
        }
        if(k == n-1) {
            sort(jobs.begin(), jobs.end());
            int min_pair = jobs[0] + jobs[1];
            int max_job = jobs[n-1];
            return max(min_pair, max_job);
        }
        typedef pair<int, vector<int>> state;
        vector<int> cost(n+1, 1e9);
        stack<state> q;
        vector<int> workers(k,0);
        q.emplace(0, workers);
        map<state, bool> vis;
        while(!q.empty()) {
            auto [u, w] = q.top();
            if(vis[q.top()]) {
                q.pop();
                continue;
            }
            vis[q.top()] = true;
            q.pop();
            int next = w[k-1];
            if(next >= cost[n]) {
                continue;
            }
            cost[u] = next;
            int v = u+1;
            if(v > n) {
                continue;
            }
            for(int i = w.size()-1;i>=0;i--) {
                auto nextw = w;
                nextw[i] += jobs[v-1];
                sort(nextw.begin(), nextw.end());
                q.emplace(v, nextw);
            }
        }
        return cost[n];
    }
};