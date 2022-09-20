class Solution {
public:
    bool canFinish(int n, vector<vector<int>>& prerequisites) {
        const int INF = 2001;
        vector<int> vis(n);
        map<int, vector<int>> prev;
        map<int, vector<int>> next;
        for(int i = 0;i<n;i++) {
            vis[i] = i;
        }
        for(const auto& x: prerequisites) {
            next[x[0]].push_back(x[1]);
            prev[x[1]].push_back(x[0]);
        }
        while(!vis.empty()) {
            int i = 0;
            while(i < vis.size() && prev[vis[i]].size() != 0) {
                i++;
            }
            // can not learn anything
            if(i >= vis.size()) {
                return false;
            }
            int u = vis[i];
            //cout<<"learn couse "<<u<<endl;
            for(const auto& v: next[u]) {
                prev[v].erase(find(prev[v].begin(), prev[v].end(), u));
            }
            vis.erase(vis.begin() + i);
        }
        return true;
    }
    vector<int> findOrder(int n, vector<vector<int>>& prerequisites) {
        const int INF = 2001;
        vector<int> ret;
        vector<int> vis(n);
        map<int, vector<int>> prev;
        map<int, vector<int>> next;
        for(int i = 0;i<n;i++) {
            vis[i] = i;
        }
        for(const auto& x: prerequisites) {
            next[x[1]].push_back(x[0]);
            prev[x[0]].push_back(x[1]);
        }
        while(!vis.empty()) {
            int i = 0;
            while(i < vis.size() && prev[vis[i]].size() != 0) {
                i++;
            }
            // can not learn anything
            if(i >= vis.size()) {
                return {};
            }
            int u = vis[i];
            ret.push_back(u);
            //cout<<"learn couse "<<u<<endl;
            for(const auto& v: next[u]) {
                prev[v].erase(find(prev[v].begin(), prev[v].end(), u));
            }
            vis.erase(vis.begin() + i);
        }
        return ret;
    }
};