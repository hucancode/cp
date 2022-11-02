class Solution {
public:
    bool canMutate(string& a, string& b) {
        bool diff = false;
        for(int i = 0;i<a.size();i++) {
            if(a[i] == b[i]) continue;
            if(diff) {
                return false;
            }
            diff = true;
        }
        return true;
    }
    int bfs(vector<string>& bank, int start, int end) {
        int n = bank.size();
        vector<int> dist(n, -1);
        queue<int> q;
        q.push(start);
        dist[start] = 0;
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            for(int v = 0;v<n;v++) {
                if(dist[v] != -1) {
                    continue;
                }
                if(canMutate(bank[u], bank[v])) {
                    dist[v] = dist[u]+1;
                    q.push(v);
                }
            }
        }
        return dist[end];
    }
    int minMutation(string start, string end, vector<string>& bank) {
        int u = 0, v = 0;
        auto it = find(bank.begin(), bank.end(), start);
        if(it == bank.end()) {
            u = bank.size();
            bank.push_back(start);
        } else {
            u = distance(bank.begin(), it);
        }
        
        it = find(bank.begin(), bank.end(), end);
        if(it == bank.end()) {
            return -1;
        } else {
            v = distance(bank.begin(), it);
        }
        return bfs(bank, u, v);
    }
};