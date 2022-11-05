class Solution {
public:
    bool adj(string& a, string& b) {
        int n = a.size();
        bool diff = false;
        for(int i = 0;i<n;i++) {
            if(a[i] != b[i]) {
                if(diff) {
                    return false;
                }
                diff = true;
            }
        }
        return true;
    }
    vector<vector<string>> trace(int start, int end, vector<string>& wordList, map<int,vector<int>>& prev) {
        vector<vector<string>> ret;
        stack<int> st;
        vector<int> current;
        st.push(end);
        while(!st.empty()) {
            auto u = st.top();
            st.pop();
            if(u == -1) {
                current.pop_back();
                // backtrack
                continue;
            }
            // cout<<"check "<<u<<endl;
            if(u == start) {
                // report result
                vector<string> path;
                path.reserve(current.size()+1);
                path.push_back(wordList[start]);
                for(auto it = current.rbegin();it != current.rend();it++) {
                    path.push_back(wordList[*it]);
                }
                ret.emplace_back(move(path));
                continue;
            }
            st.push(-1);
            current.push_back(u);
            //cout<<"prev "<<u<<" "<<wordList[u]<<" size = "<<prev[u].size()<<endl;
            for(auto v: prev[u]) {
                //cout<<"push "<<wordList[v]<<endl;
                st.push(v);
            }
        }
        return ret;
    }
    void bfs(int start, int end, vector<string>& wordList, map<int,vector<int>>& prev) {
        const int INF = 2e9;
        int n = wordList.size();
        vector<int> dist(n, INF);
        vector<bool> vis(n, false);
        queue<int> q;
        q.push(start);
        dist[start] = 0;
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            int next = dist[u]+1;
            if(vis[u]) {
                continue;
            }
            vis[u] = true;
            for(int v = 0;v<n;v++) {
                if(dist[v] < next || u == v) {
                    continue;
                }
                if(!adj(wordList[u], wordList[v])) {
                    continue;
                }
                dist[v] = next;
                prev[v].push_back(u);
                q.push(v);
            }
        }
        return;
        cout<<"dist - ";
        for(auto x: dist) {
            cout<<x<<' ';
        }
        cout<<endl;
    }
    vector<vector<string>> findLadders(string beginWord, string endWord, vector<string>& wordList) {
        int n = wordList.size();
        vector<vector<string>> ret;
        auto end = distance(wordList.begin(), find(wordList.begin(), wordList.end(), endWord));
        if(end == n) {
            return ret;
        }
        auto start = distance(wordList.begin(), find(wordList.begin(), wordList.end(), beginWord));
        if(start == n) {
            wordList.push_back(beginWord);
            start = n;
            n++;
        }
        map<int, vector<int>> prev;
        bfs(start, end, wordList, prev);
        return trace(start, end, wordList, prev);
    }
};