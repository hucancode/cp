class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        if(target < 0) {
            target = -target;
            for(auto& x: nums) {
                x = -x;
            }
            auto ret = fourSum(nums, target);
            for(auto& arr: ret) {
                for(auto& x: arr) {
                    x = -x;
                }
            }
            return ret;
        }
        int n = nums.size();
        sort(nums.begin(), nums.end());
        set<vector<int>> ret;
        int i1 = 0;
        int n1 = n-3;
        for(int i1 = 0;i1<n1;i1++) {
            long long x1 = target - nums[i1];
            int n2 = distance(nums.begin(), 
                upper_bound(nums.begin()+i1+1, nums.end()-2, x1));
            for(int i2 = i1+1;i2<n2;i2++) {
                long long x2 = x1 - nums[i2];
                int n3 = distance(nums.begin(), 
                    upper_bound(nums.begin()+i2+1, nums.end()-1, x2));
                for(int i3 = i2+1;i3<n3;i3++) {
                    long long x3 = x2 - nums[i3];
                    int i4 = distance(nums.begin(), 
                        lower_bound(nums.begin()+i3+1, nums.end(), x3));
                    if(i4 >= n) continue;
                    if(nums[i4] == x3) {
                        ret.insert({nums[i1], nums[i2], nums[i3], nums[i4]});
                    }
                }
            }
        }
        vector<vector<int>> sums(ret.begin(), ret.end());
        return sums;
    }
};