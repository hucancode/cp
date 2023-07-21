typedef pair<int, int> pi;
class Solution {
public:
    int findNumberOfLIS(vector<int>& nums) {
        int n = nums.size();
        vector<pi> f(n);
        f[0] = {1,1};
        for(int i = 1;i<n;i++) {
            int len = 1;
            int count = 1;
            for(int j=0;j<i;j++) {
                if(nums[j] < nums[i]) {
                    int nextLen = f[j].first + 1;
                    int nextCount = f[j].second;
                    if(nextLen > len) {
                        len = nextLen;
                        count = nextCount;
                    } else if(nextLen == len) {
                        count += nextCount;
                    }
                }
            }
            f[i] = {len, count};
        }
        int bestLen = 0;
        int ret = 0;
        for(int i = 0;i<n;i++) {
            if(f[i].first > bestLen) {
                bestLen = f[i].first;
                ret = f[i].second;
            } else if(f[i].first == bestLen) {
                ret += f[i].second;
            }
        }
        return ret;
    }
};