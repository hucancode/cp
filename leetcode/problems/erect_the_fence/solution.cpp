class Solution {
public:
    int side(vector<int> a, vector<int> b, vector<int> c) {
        int ret = (b[0] - a[0]) * (c[1] - a[1]) - 
            (b[1] - a[1]) * (c[0] - a[0]);
        return ret > 0?1:ret<0?-1:0;
    }
    float lenSqr(vector<int> a, vector<int> b) {
        int dx = b[0] - a[0];
        int dy = b[1] - a[1];
        return dx*dx + dy*dy;
    }
    vector<vector<int>> outerTrees(vector<vector<int>>& trees) {
        int n = trees.size();
        if(trees.size() < 4) {
            return trees;
        }
        int leftMost = 0; // left most tree
        for(int i = 1;i<n;i++) {
            if(trees[i][0] < trees[leftMost][0]) {
                leftMost = i;
            }
        }
        set<int> hull;
        vector<bool> vis(n, false);
        hull.insert(leftMost);
        int a = leftMost;
        while(true) {
            int b = 0;
            while(b == a || (b < n && vis[b])) {
                b++;
            }
            if(b == n) {
                break;
            }
            vector<int> bline = {b};
            for(int c = b+1;c<n;c++) {
                if(vis[c] || c == a || c == b) {
                    continue;
                }
                int k = side(trees[a], trees[b], trees[c]);
                // cout<<"check t["<<a<<"] "<<trees[a][0]<<"-"<<trees[a][1]
                //     <<", t["<<b<<"] "<<trees[b][0]<<"-"<<trees[b][1]
                //     <<", t["<<c<<"] "<<trees[c][0]<<"-"<<trees[c][1]<<", side = "<<k<<endl;
                if(k == 1) {
                    bline.clear();
                    bline.push_back(c);
                    b = c;
                } else if(k == 0) {
                    bline.push_back(c);
                    float ab = lenSqr(trees[a],trees[b]);
                    float ac = lenSqr(trees[a],trees[c]);
                    if(ac > ab) {
                        b = c;
                    }
                }
            }
            for(auto x: bline) {
                hull.insert(x);
                vis[x] = true;
            }
            a = b;
            if(a == leftMost) {
                break;
            }
        }
        vector<vector<int>> ret;
        for(auto i: hull) {
            ret.push_back(trees[i]);
        }
        return ret;
    }
};