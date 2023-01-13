class Solution {
public:
    void buildPermutation(vector<int>& nums, vector<int>& a, int i, int n) {
        int k = n-i;
        int size = (1<<k);
        a.resize(size,0);
        for(int j = 0;j<size;j++) {
            for(int b = 0;b<k;b++) {
                if((1<<b)&j) {
                    continue;
                }
                int next = (1<<b)|j;
                a[next] += nums[i+b];
            }
        }
        sort(a.begin(), a.end());
    }
    int minAbsDifference(vector<int>& nums, int goal) {
        int n = nums.size();
        int m = n/2;
        vector<int> left, right;
        buildPermutation(nums, left, 0, m);
        buildPermutation(nums, right, m, n);
        int ret = 2e9+1;
        int j = right.size();
        for(auto a: left) {
            int target = goal - a;
            j = distance(right.begin(), lower_bound(right.begin(), right.begin()+j, target));
            if(j < right.size()) {
                int b = right[j];
                ret = min(ret, abs(a + b - goal));
            }
            if(j != 0) {
                int b = right[j-1];
                ret = min(ret, abs(a + b - goal));
            }
            if(ret == 0) {
                break;
            }
            j = right.size();
        }
        return ret;
    }
};