class Solution {
    vector<int> parent;
public:
    int find(int a) {
        while(parent[a] != a) {
            a = parent[a];
        }
        return a;
    }
    void unionAB(int a, int b) {
        auto pa = find(a);
        auto pb = find(b);
        if(pa > pb) {
            swap(pa, pb);
        }
        parent[pb] = pa;
    }
    string smallestEquivalentString(string s1, string s2, string baseStr) {
        parent.resize(26);
        iota(parent.begin(), parent.end(), 0);
        int n = s1.size();
        for(int i = 0;i<n;i++) {
            unionAB(s1[i] - 'a', s2[i] - 'a');
        }
        for(auto& c: baseStr) {
            c = find(c - 'a') + 'a';
        }
        return baseStr;
    }
};