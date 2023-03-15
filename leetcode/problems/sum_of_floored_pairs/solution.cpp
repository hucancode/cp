class Solution {
public:
    int sumOfFlooredPairs(vector<int>& nums) {
        const int INF = 1e9+7;
        sort(nums.begin(), nums.end());
        long long ret = 0;
        int n = nums.size();
        //if(n > 1000) return 0;
        for(auto i = 0;i<n;) {
            //cout<<"check "<<nums[i]<<endl;
            auto j = distance(nums.begin(), upper_bound(nums.begin()+i, nums.end(), nums[i]));
            auto count = j-i;
            //cout<<"there are "<<count<<" number "<<nums[i]<<endl;
            ret += count * count;
            ret %= INF;
            while(j != n) {
                auto k = nums[j]/nums[i];
                auto target = nums[i]*(k+1);
                //cout<<"find "<<target<<endl;
                auto next = distance(nums.begin(), lower_bound(nums.begin()+j, nums.end(), target));
                //cout<<"found at "<<next<<endl;
                auto score = (next-j)*k;
                ret += score*count;
                ret %= INF;
                //cout<<"ret = "<<ret<<endl;
                j = next;
            }
            i += count;
        }
        return ret;
    }
};