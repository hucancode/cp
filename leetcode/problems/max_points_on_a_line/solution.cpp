class Solution {
public:
    int maxPoints(vector<vector<int>>& points) {
        int n = points.size();
        if(n < 3) {
            return n;
        }
        map<pair<float,float>, set<int>> lines;
        map<int, set<int>> xlines;
        map<int, set<int>> ylines;
        for(int i = 0;i<n-1;i++) {
            float x1 = points[i][0];
            float y1 = points[i][1];
            for(int j = i+1;j<n;j++) {
                float x2 = points[j][0];
                float y2 = points[j][1];
                if(x1 == x2) {
                    xlines[x1].insert(i);
                    xlines[x1].insert(j);
                    continue;
                }
                if(y1 == y2) {
                    ylines[y1].insert(i);
                    ylines[y1].insert(j);
                    continue;
                }
                float a = (y1 - y2)/(x1 - x2);
                float b = y1 - a*x1;
                lines[make_pair(a,b)].insert(i);
                lines[make_pair(a,b)].insert(j);
            }
        }
        int ret = 2;
        for(auto line: lines) {
            ret = max(ret, (int)line.second.size());
        }
        for(auto x: xlines) {
            ret = max(ret, (int)x.second.size());
        }
        for(auto y: ylines) {
            ret = max(ret, (int)y.second.size());
        }
        return ret;
    }
};