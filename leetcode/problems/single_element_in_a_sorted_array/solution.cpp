class Solution {
public:
    int singleNonDuplicate(vector<int>& nums) {
        if(nums.size() == 1) {
            return nums[0];
        }
        int l = 0, r = nums.size() - 1;
        while(l<r) {
            int m = (r+l)/2;
            int wl = m-l;
            int wr = r-m;
            int ml = m-1;
            int mr = m+1;
            if(nums[m] == nums[mr]) {
                wr++;
            } else if (nums[m] == nums[ml]){
                wl++;
            }
            if(wl%2==0 && wr%2 == 0) {
                return nums[m];
            }
            if(wl%2 == 0) {
                l = m;
                if (nums[m] == nums[ml]) {
                    l++;
                }
            } else if(wr%2 == 0) {
                r = m;
                if (nums[m] == nums[mr]) {
                    r--;
                }
            }
        }
        return nums[l];
    }
};