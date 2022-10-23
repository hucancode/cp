class Solution {
public:
    long long makeSimilar(vector<int>& nums, vector<int>& target) {
        int n = nums.size();
        sort(nums.begin(), nums.end());
        sort(target.begin(), target.end());
        long long p = 0;
        long long m = 0;
        int i = 0;
        int j = 0;
        while(i < n && j < n) {
            if(target[i]%2 == 0) {
                i++;
                continue;
            }
            if(nums[j]%2 == 0) {
                j++;
                continue;
            }
            int d = target[i] - nums[j];
            if(d < 0) {
                m += -d/2;
            }
            if(d > 0) {
                p += d/2;
            }
            i++;
            j++;
        }
        i = 0;
        j = 0;
        while(i < n && j < n) {
            if(target[i]%2 == 1) {
                i++;
                continue;
            }
            if(nums[j]%2 == 1) {
                j++;
                continue;
            }
            int d = target[i] - nums[j];
            if(d < 0) {
                m += -d/2;
            }
            if(d > 0) {
                p += d/2;
            }
            i++;
            j++;
        }
        return (p+m)/2;
    }
};