class Solution {
public:
    int maxNonDecreasingLength(vector<int>& nums1, vector<int>& nums2) {
        int n = nums1.size();
        vector<int> f1(n,1);
        vector<int> f2(n,1);
        int ret = 1;
        for(int i = 1;i<n;i++) {
            if(nums1[i] >= nums1[i-1]) {
                f1[i] = max(f1[i], f1[i-1]+1);
            }
            if(nums1[i] >= nums2[i-1]) {
                f1[i] = max(f1[i], f2[i-1]+1);
            }
            if(nums2[i] >= nums1[i-1]) {
                f2[i] = max(f2[i], f1[i-1]+1);
            }
            if(nums2[i] >= nums2[i-1]) {
                f2[i] = max(f2[i], f2[i-1]+1);
            }
            ret = max(ret, f1[i]);
            ret = max(ret, f2[i]);
        }
        return ret;
    }
};