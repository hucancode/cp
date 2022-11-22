class Solution {
public:
    int minCost(int maxTime, vector<vector<int>>& edges, vector<int>& passingFees) {
        int n = passingFees.size();
        map<int,vector<tuple<int,int,int>>> mp;
        for(auto edge: edges) {
            int x = edge[0];
            int y = edge[1];
            int time = edge[2];
            if(y != 0) {
                mp[x].emplace_back(passingFees[y], time, y);
            }
            if(x != 0) {
                mp[y].emplace_back(passingFees[x], time, x);
            }
        }
        vector<int> dist(n, 2e9);
        vector<bool> vis(n, false);
        typedef tuple<int,int,int> state; // time, fee, city
        priority_queue<state, vector<state>, greater<state>> q;
        q.emplace(0,passingFees[0],0);
        while(!q.empty()) {
            int currentTime,currentFee,u;
            tie(currentTime,currentFee,u) = q.top();
            q.pop();
            if(currentFee >= dist[u]) {
                continue;
            }
            dist[u] = currentFee;
            //cout<<"travel "<<u<<endl;
            //cout<<"dist["<<u<<"]="<<dist[u]<<endl;
            if(u == n-1) {
                continue;
            }
            for(auto edge: mp[u]) {
                int v, time, fee;
                tie(fee, time, v) = edge;
                int nextTime = currentTime + time;
                int nextFee = currentFee + fee;
                if(nextFee >= dist[v]) {
                    continue;
                }
                if(nextTime > maxTime) {
                    continue;
                }
                q.emplace(nextTime, nextFee, v);
            }
        }
        if(dist[n-1] == 2e9) {
            return -1;
        }
        return dist[n-1];
    }
};