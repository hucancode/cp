class Solution {
public:
    bool checkSubarraySum(vector<int>& nums, int k) {
        int n = nums.size();
        vector<int> prefixSum(n+1);
        prefixSum[0] = 0;
        for(int i = 1;i<=n;i++) {
            prefixSum[i] = prefixSum[i-1]+nums[i-1];
        }
        for(int i = 2;i<=n;i++) {
            int m = (prefixSum[i] - prefixSum[i-2])/k;
            int target = prefixSum[i] - m*k;
            auto tail = prefixSum.begin()+i-1;
            while(target >= 0 && tail != prefixSum.begin()) {
                auto it = lower_bound(prefixSum.begin(), tail, target);
                //cout<<"at prefix["<<i<<"], num = "<<nums[i-1]<<", search for "<<target<<" in [0,"<<distance(prefixSum.begin(), tail)<<")"<<endl;
                if(it != tail && *it == target) {
                    //cout<<"found ! at "<<distance(prefixSum.begin(), tail)<<endl;
                    return true;
                }
                m++;
                target = prefixSum[i] - m*k;
            }
        }
        return false;
    }
};