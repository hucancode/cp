class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        set<int> buffer;
        for(int i = 0;i<nums.size();i++) {
            if(i > k) buffer.erase(nums[i-k-1]);
            buffer.insert(nums[i]);
            if(buffer.size() <= min(i,k)) return true;
        }
        return false;
    }
};