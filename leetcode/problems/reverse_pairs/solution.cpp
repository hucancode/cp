class Solution {
    int ret;
public:
    void mergeSort(vector<int>::iterator first, vector<int>::iterator last) {
        if (last - first <= 1) {
            return;
        }
        auto middle = first + (last - first) / 2;
        mergeSort(first, middle);
        mergeSort(middle, last);
        for(auto it = first;it!= middle;it++) {
            int target;
            auto k = *it;
            if(k < 0) {
                target = k/2;
            } else {
                target = ((long long)k+1)/2;
            }
            auto count = distance(middle, lower_bound(middle, last, target));
            ret += count;
        }
        inplace_merge(first, middle, last);
    }
    int reversePairs(vector<int>& nums) {
        int n = nums.size();
        ret = 0;
        mergeSort(nums.begin(), nums.end());
        return ret;
    }
};