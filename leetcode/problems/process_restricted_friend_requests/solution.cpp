class Solution {
public:
    vector<int> parent;
    int find(int x) {
        if(parent[x] == x) {
            return x;
        }
        return find(parent[x]);
    }

    void unionXY(int x, int y) {
        int px = find(x);
        int py = find(y);
        parent[px] = py;
    }

    bool canUnion(int x, int y, vector<vector<int>>& restrictions) {
        int px = find(x);
        int py = find(y);
        if(px == py) {
            return true;
        }
        for(auto& r: restrictions) {
            int a = r[0];
            int b = r[1];
            if((a == px && b == py) || (a == py && b == px)) {
                return false;
            }
        }
        return true;
    }
    vector<bool> friendRequests(int n, vector<vector<int>>& restrictions, vector<vector<int>>& requests) {
        parent.resize(n);
        iota(parent.begin(), parent.end(), 0);
        vector<bool> ret;
        ret.reserve(requests.size());
        for(auto r: requests) {
            int x = r[0];
            int y = r[1];
            if(canUnion(x,y, restrictions)) {
                ret.push_back(true);
                unionXY(x,y);
                for(auto& r: restrictions) {
                    r[0] = find(r[0]);
                    r[1] = find(r[1]);
                }
            } else {
                ret.push_back(false);
            }
        }
        return ret;
    }
};