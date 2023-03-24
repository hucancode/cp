class Solution {
public:
    bool check(vector<int>& nums, int cap, int requirement) {
        int n = nums.size();
        int f1 = nums[0] <= cap;
        int f2 = nums[1] <= cap;
        f2 = max(f1, f2);
        for(int i = 2;i<n;i++) {
            int f = f2;
            if(nums[i] <= cap) {
                f = max(f1+1, f);
            }
            f1 = f2;
            f2 = f;
            if(f2 >= requirement) {
                return true;
            }
        }
        return f2 >= requirement;
    }
    int minCapability(vector<int>& nums, int k) {
        if(nums.size() == 1) {
            return nums[0];
        }
        int l = 1, r = *max_element(nums.begin(), nums.end());
        while(l < r) {
            auto m = l+(r-l)/2;
            auto good = check(nums, m, k);
            if(good) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
};