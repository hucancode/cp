class Solution {
public:
    int beautifulSubsets(vector<int>& nums, int k) {
        int n = nums.size();
        int ret = 0;
        queue<pair<int, int>> q;
        q.emplace(0,0);
        while(!q.empty()) {
            int i, mask;
            tie(i, mask) = q.front();
            q.pop();
            if(i == n) {
                if(mask) {
                    ret++;
                }
                continue;
            }
            bool valid = true;
            for(int j = 0;j<i;j++) {
                if((mask & (1<<j)) && abs(nums[j] - nums[i]) == k) {
                    valid = false;
                    break;
                }
            }
            if(valid) {
                q.emplace(i+1, mask | (1<<i));
            }
            q.emplace(i+1, mask);
        }
        return ret;
    }
};