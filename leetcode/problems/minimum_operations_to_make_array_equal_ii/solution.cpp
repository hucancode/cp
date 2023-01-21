class Solution {
public:
    long long minOperations(vector<int>& nums1, vector<int>& nums2, int k) {
        int n = nums1.size();
        if(k == 0) {
            for(int i = 0;i<n;i++) {
                if(nums1[i] != nums2[i]) {
                    return -1;
                }
            }
            return 0;
        }
        long long op1 = 0;
        long long op2 = 0;
        for(int i = 0;i<n;i++) {
            int d = nums1[i] - nums2[i];
            if(d%k != 0) {
                return -1;
            }
            if(d > 0) {
                op1 += d/k;
            } else {
                op2 += -d/k;
            }
        }
        if(op1 != op2) {
            return -1;
        }
        return op1;
    }
};