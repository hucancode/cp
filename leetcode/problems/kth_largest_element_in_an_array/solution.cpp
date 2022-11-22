class Solution {
public:
    // O(n)
    inline int valToKey(int v) {
        return v + 1e5;
    }
    inline int keyToVal(int k) {
        return k - 1e5;
    }
    int countingSort(vector<int>& nums, int k) {
        int n = 2e5+1;
        vector<int> count(n+1,0);
        int i;
        for(const auto& val:nums) {
            i = valToKey(val);
            count[i]++;
        }
        i = n;
        while(k > 0) {
            k -= count[i--];
        }
        return keyToVal(i)+1;
    }
    // O(nlogn)
    int quickSort(vector<int>& nums, int k) {
        sort(nums.rbegin(), nums.rend());
        return nums[k-1];
    }
    int findKthLargest(vector<int>& nums, int k) {
        return countingSort(nums, k);
    }
};