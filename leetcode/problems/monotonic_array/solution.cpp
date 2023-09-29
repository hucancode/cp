class Solution {
public:
    bool isMonotonic(vector<int>& nums) {
        int n = nums.size();
        for(int i = 1;i<n;i++)
            if((nums[i] != nums[i-1]) && 
                ((nums[i] > nums[i-1]) != (nums[n-1] > nums[0]))) return false;
        return true;
    }
};