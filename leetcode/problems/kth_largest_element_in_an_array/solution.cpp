class Solution {
public:
    // O(n+maxk*2)
    int countingSort(vector<int>& nums, int k) {
        vector<int> count(20002,0);
        int i;
        for(const auto& val:nums) {
            i = val + 10000;
            count[i]++;
        }
        i = 20001;
        while(k > 0) {
            k -= count[i--];
        }
        return i-10000+1;
    }
    // O(nlogn)
    int quickSort(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end(), greater<int>());
        return nums[k-1];
    }
    int findKthLargest(vector<int>& nums, int k) {
        return countingSort(nums, k);
    }
};