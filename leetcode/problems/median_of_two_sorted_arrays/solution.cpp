class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        nums1.reserve(nums1.size() + nums2.size());
        for(const auto& x: nums2) {
            nums1.insert(upper_bound(nums1.begin(), nums1.end(), x), x);
        }
        int n = nums1.size();
        if(n == 0) {
            return 0;
        }
        if(n == 1) {
            return nums1[0];
        }
        if(n%2 == 1) {
            return nums1[n/2];
        }
        return (nums1[n/2] + nums1[n/2 - 1])/2.0;
    }
};