class Solution {
public:
    bool checkArray(vector<int>& nums, int k) {
        int n = nums.size();
        queue<int> pos;
        queue<int> v;
        int x = 0;
        for(int i = 0;i<n;i++) {
            while(!pos.empty() && (i - pos.front() >= k)) {
                pos.pop();
                x -= v.front();
                v.pop();
            }
            nums[i] -= x;
            if(nums[i] < 0) {
                return false;
            }
            if(nums[i] > 0) {
                if(n-i < k) {
                    return false;
                }
                pos.push(i);
                v.push(nums[i]);
                x += nums[i];
                nums[i] = 0;
            }
        }
        return true;
    }
};