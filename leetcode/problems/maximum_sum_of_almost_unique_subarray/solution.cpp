class Solution {
public:
    long long maxSum(vector<int>& nums, int m, int k) {
        int n = nums.size();
        map<int, int> count;
        long long ret = 0;
        long long sum = 0;
        int distinct = 0;
        for(int i = 0;i<k;i++) {
            int x = nums[i];
            if(count[x] == 0) {
                distinct++;
            }
            count[x]++;
            sum += x;
        }
        if(distinct >= m) {
            ret = max(ret, sum);
        }
        for(int i = k;i<n;i++) {
            int y = nums[i-k];
            int x = nums[i];
            count[y]--;
            if(count[y] == 0) {
                distinct--;
            }
            sum -= y;
            if(count[x] == 0) {
                distinct++;
            }
            count[x]++;
            sum += x;
            if(distinct >= m) {
                ret = max(ret, sum);
            }
        }
        return ret;
    }
};