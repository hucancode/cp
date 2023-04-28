class Solution {
public:
    int numSimilarGroups(vector<string>& strs) {
        int n = strs.size();
        vector<int> parent(n);
        iota(parent.begin(), parent.end(), 0);
        auto similar = [](string& a, string& b) {
            int diff = 0;
            for(int i = 0;i<a.size();i++) {
                diff += a[i] != b[i];
                if(diff > 2) {
                    return false;
                }
            }
            return true;
        };
        auto find = [&](int x) {
            while(parent[x] != x) {
                x = parent[x];
            }
            return x;
        };
        auto unionXY = [&](int x, int y) {
            auto px = find(x);
            auto py = find(y);
            parent[py] = px;
        };
        for(int i = 0;i<n;i++) {
            for(int j = i+1;j<n;j++) {
                if(find(i) != find(j) && similar(strs[i], strs[j])) {
                    unionXY(i, j);
                }
            }
        }
        set<int> st;
        for(int i = 0;i<n;i++) {
            st.insert(find(i));
        }
        return st.size();
    }
};