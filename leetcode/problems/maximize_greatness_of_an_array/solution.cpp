class Solution {
public:
    int maximizeGreatness(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<int> count;
        for(auto i = nums.begin();i!= nums.end();) {
            auto next = upper_bound(nums.begin(), nums.end(),*i);
            auto c = distance(i, next);
            count.push_back(c);
            i = next;
        }
        int n = count.size();
        int k = count[n-1];
        int j = n-2;
        int ret = 0;
        while(j >= 0) {
            ret += min(k, count[j]);
            k = max(k, count[j]);
            j--;
        }
        return ret;
    }
};