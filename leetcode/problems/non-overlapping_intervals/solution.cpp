class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& intervals) {
        auto cmp = [](const vector<int>& a, const vector<int>& b) {
            return a[1] < b[1];
        };
        sort(intervals.begin(), intervals.end(), cmp);
        // for(const auto& x: intervals) {
        //     cout<<x[0]<<"~"<<x[1]<<" ";
        // }
        // cout<<endl;
        int pivot = intervals[0][1];
        int ret = 0;
        for(int i = 1;i<intervals.size();i++) {
            bool overlap = intervals[i][0] < pivot;
            if(overlap) {
                ret++;
                continue;
            }
            pivot = intervals[i][1];
        }
        return ret;
    }
};