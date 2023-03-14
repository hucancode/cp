class Solution {
public:
    int smallestDistancePair(vector<int>& nums, int k) {
        int n = nums.size();
        vector<int> d(1e6+1, 0);
        for(int i = 0;i<n-1;i++) {
            for(int j = i+1;j<n;j++) {
                d[abs(nums[i] - nums[j])]++;
            }
        }
        int i = 0;
        while(d[i] < k) {
            k -= d[i++];
        }
        return i;
    }
};