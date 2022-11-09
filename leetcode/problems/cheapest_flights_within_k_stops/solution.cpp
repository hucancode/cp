class Solution {
public:
    int findCheapestPrice(int n, vector<vector<int>>& flights, int src, int dst, int k) {
        k+=2;
        map<int, vector<pair<int,int>>> mp;
        for(auto flight: flights) {
            mp[flight[0]].emplace_back(flight[1], flight[2]);
        }
        vector<vector<int>> cost(n, vector<int>(k,1e7));
        cost[src][0] = 0;
        queue<tuple<int,int,int>> q;
        q.emplace(src, 0, 0);
        int ret = 1e7;
        while(!q.empty()) {
            int u, jump, len;
            tie(u, jump, len) = q.front();
            q.pop();
            //cout<<"check "<<u<<"("<<jump<<") cost = "<<len<<endl;
            if(jump >= k) {
                continue;
            }
            if(len >= ret) {
                continue;
            }
            if(*min_element(cost[u].begin()+1, cost[u].begin()+jump+1) <= len) {
                continue;
            }
            cost[u][jump] = min(cost[u][jump], len);
            if(u == dst) {
                ret = min(ret, cost[u][jump]);
                continue;
            }
            for(auto flight: mp[u]) {
                auto v = flight.first;
                auto duv = flight.second;
                //cout<<"push "<<v<<endl;
                q.emplace(v, jump+1, cost[u][jump] + duv);
            }
        }
        if(ret == 1e7) {
            ret = -1;
        }
        return ret;
    }
};