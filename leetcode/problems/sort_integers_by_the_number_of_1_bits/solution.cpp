class Solution {
public:
    vector<int> sortByBits(vector<int>& arr) {
        sort(arr.begin(), arr.end(), [](int a, int b) {
          int pa = __builtin_popcount(a);
          int pb = __builtin_popcount(b);
          return pa < pb || (pa == pb && a < b);
        });
        return arr;
    }
};