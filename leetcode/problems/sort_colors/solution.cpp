class Solution {
public:
    void sortColors(vector<int>& nums) {
        const int MAX_VALUE = 3;
        vector<int> f(MAX_VALUE, 0);
        int n = nums.size();
        for(int i = 0;i<n;i++) {
            f[nums[i]]++;
        }
        auto it = nums.begin();
        for(int i = 0;i<f.size();i++) {
            fill(it, it+f[i], i);
            it += f[i];
        }
    }
};