class Solution {
public:
    int maxWidthOfVerticalArea(vector<vector<int>>& points) {
        vector<int> px(points.size());
        transform(begin(points), end(points), begin(px), [](const auto& a) { return a[0]; });
        sort(begin(px), end(px));
        adjacent_difference(begin(px), end(px), begin(px));
        return *max_element(next(begin(px)), end(px));
    }
};