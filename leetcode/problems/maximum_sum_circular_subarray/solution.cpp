class Solution {
public:
    int maxSubarraySumCircular(vector<int>& nums) {
        int n = nums.size();
        vector<int> sum(n*2);
        // sum[i] = sum of nums[0]~nums[i]
        sum[0] = nums[0];
        int ret = nums[0];
        for(int i = 1;i<n*2;i++) {
            int j = i%n;
            sum[i] = sum[i-1] + nums[j];
        }
        // find i and j where sum[i] - sum[j] max, i - j must be smaller than n
        // travese the array, store candidates in a queue
        // when a candidate too far from current positition, discard
        // when a new candidate k goes in, discard all existing candidate has value >= nums[k]
        deque<int> q;
        q.push_back(0);
        for(int i = 1;i<n*2;i++) {
            if(i - q.front() > n) {
                q.pop_front();
            }
            auto delta = sum[i] - sum[q.front()];
            ret = max(ret, delta);
            while(!q.empty() && sum[q.back()] >= sum[i] ) {
                q.pop_back();
            }
            q.push_back(i);
        }
        return ret;
    }
};