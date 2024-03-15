class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> left(n);
        vector<int> right(n);
        exclusive_scan(nums.begin(), nums.end(),
                        left.begin(),
                        1, multiplies<int>());
        exclusive_scan(nums.rbegin(), nums.rend(),
                        right.rbegin(),
                        1, multiplies<int>());
        transform (left.begin(), left.end(), 
            right.begin(), right.begin(), 
            multiplies<int>());
        return right;
    }
};
