class Solution {
public:
    void mergeSort(vector<int>::iterator l, vector<int>::iterator r) {
        int d = distance(l, r);
        if(d <= 1) return;
        auto m = l + d/2;
        mergeSort(l, m);
        mergeSort(m, r);
        // this inplace merge algorithm is too complex, can't implement
        inplace_merge(l, m, r);
    }
    vector<int> sortArray(vector<int>& nums) {
        mergeSort(nums.begin(), nums.end());
        return nums;
    }
};