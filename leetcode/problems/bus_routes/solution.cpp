class Solution {
public:
    bool isConnected(vector<int>& a, vector<int>& b) {
        int i = 0;
        int j = 0;
        while(i < a.size() && j < b.size()) {
            if(a[i] == b[j]) {
                return true;
            }
            if(a[i] > b[j]) {
                j++;
            } else {
                i++;
            }
        }
        return false;
    }
    bool hasNode(vector<int>& a, int node) {
        return binary_search(a.begin(), a.end(), node);
    }
    int numBusesToDestination(vector<vector<int>>& routes, int source, int target) {
        const int INF = 1000001;
        int n = routes.size();
        map<int, vector<int>> edges;
        vector<int> vis(n);
        vector<int> dist(n, INF);
        // build edge list
        for(int i = 0;i<n ;i++) {
            sort(routes[i].begin(), routes[i].end());
        }
        for(int i = 0;i<n -1 ;i++) {
            for(int j = i+1;j<n ;j++) {
                if(isConnected(routes[i], routes[j])) {
                    edges[i].push_back(j);
                    edges[j].push_back(i);
                }
            }
        }
        // setup
        for(int i = 0;i<n ;i++) {
            vis[i] = i;
            if(hasNode(routes[i], source)) {
                dist[i] = 1;
                if(source == target) {
                    dist[i] = 0;
                }
            }
        }
        // dijkstra
        while(!vis.empty()) {
            int i = 0;
            int u = vis[i];
            int du = dist[i];
            for(int j = 1;j<vis.size();j++) {
                int v = vis[j];
                int dv = dist[j];
                if(dv < du) {
                    i = j;
                    u = v;
                    du = dv;
                }
            }
            if(du == INF) {
                break;
            }
            //cout<<"u = "<<u<<", now updating all neighbor of u ("<<edges[u].size()<<" node)"<<endl;
            if(hasNode(routes[u], target)) {
                return du;
            }
            for(int j = 0;j<vis.size();j++) {
                auto v = vis[j];
                bool hasEdge = hasNode(edges[u], v);
                if(hasEdge) {
                    dist[j] = min(dist[j], dist[i] + 1);
                    //cout<<"update, v = "<<v<<" now has dist = "<<dist[i]<<endl;
                }
            }
            vis.erase(vis.begin() + i);
            dist.erase(dist.begin() + i);
        }
        return -1;
    }
};