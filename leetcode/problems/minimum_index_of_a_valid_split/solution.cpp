class Solution {
public:
    int minimumIndex(vector<int>& nums) {
        int n = nums.size();
        int m, count = 0;
        for(auto x: nums) {
            if(count == 0) {
                m = x;
                count = 1;
            } else {
                if (m == x) {
                    count++;
                } else {
                    count--;
                }
            }
        }
        vector<int> prefix(n+1, 0);
        for(int i = 1;i<=n;i++) {
            prefix[i] = prefix[i-1] + (nums[i-1] == m?1:0);
            count = prefix[i];
        }
        for(int i = 1;i<n;i++) {
            int x = prefix[i];
            int y = count - x;
            if(x*2 > i && y*2 > n-i) {
                return i-1;
            }
        }
        return -1;
    }
};