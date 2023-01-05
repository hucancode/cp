class Solution {
public:
    int findMinArrowShots(vector<vector<int>>& points) {
        sort(points.begin(), points.end());
        int ret = 0;
        int last = points[0][1];
        int n = points.size();
        for(int i = 1;i<n;i++) {
            int head = points[i][0];
            int tail = points[i][1];
            if(head > last) {
                ret++;
                last = tail;
            } else {
                last = min(last, tail);
            }
        }
        ret++;
        return ret;
    }
};