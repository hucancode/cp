class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        int n = nums.size();
        int ret = nums[0]+nums[1]+nums[2];
        int delta = ret - target;
        for(int i = 0;i<n-2;i++) {
            for(int j = i+1;j<n-1;j++) {
                int k = j+1;
                auto sum2 = nums[i] + nums[j];
                auto ideal = target - sum2;
                auto actual = lower_bound(nums.begin()+k, nums.end(), ideal);
                if(actual == nums.end()) {
                    actual--;
                }
                int d = *actual - ideal;
                if(abs(d) < abs(delta)) {
                    ret = sum2 + *actual;
                    delta = d;
                }
            }
            if(delta == 0) {
                break;
            }
        }
        return ret;
    }
};