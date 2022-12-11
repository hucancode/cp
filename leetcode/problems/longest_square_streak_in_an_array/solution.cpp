class Solution {
public:
    int longestSquareStreak(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        int ret = -1;
        int n = nums.size();
        map<int,int> streak;
        streak[nums[0]] = 1;
        for(int i=1;i<n;i++) {
            int x = nums[i];
            int k = sqrt(x);
            if(k*k != x) {
                streak[x] = max(streak[x], 1);
            } else {
                streak[x] = max(streak[x], streak[k]+1);
            }
            if(streak[x] >= 2) {
                ret = max(streak[x], ret);
            }
        }
        return ret;
    }
};