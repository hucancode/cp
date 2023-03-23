class Union {
    vector<int> group;
public:
    Union(int n) {
        group.resize(n);
        iota(group.begin(), group.end(), 0);
    }
    int find(int x) {
        if(group[x] == x) return x;
        return find(group[x]);
    }
    void unionXY(int x, int y) {
        auto px = find(x);
        auto py = find(y);
        group[px] = py;
    }
};
class Solution {
public:
    int makeConnected(int n, vector<vector<int>>& connections) {
        int cable = connections.size();
        if(cable < n - 1) {
            return -1;
        }
        Union u(n);
        for(auto c: connections) {
            u.unionXY(c[0], c[1]);
        }
        set<int> component;
        for(int i = 0;i<n;i++) {
            component.insert(u.find(i));
        }
        return component.size() - 1;
    }
};