class Solution {
public:
    bool increasingTriplet(vector<int>& nums) {
        int n = nums.size();
        if(n < 3) {
            return false;
        }
        int minLeft;
        vector<int> maxRight(n);
        maxRight[n-1] = nums[n-1];
        for(int i = n-2;i>=0;i--) {
            maxRight[i] = max(maxRight[i+1], nums[i]);
        }
        minLeft = nums[0];
        for(int i = 1;i<n-1;i++) {
            if(minLeft < nums[i] && maxRight[i+1] > nums[i]) {
                return true;
            }
            minLeft = min(minLeft, nums[i]);
        }
        return false;
    }
};