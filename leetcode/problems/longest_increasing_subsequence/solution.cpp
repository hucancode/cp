class Solution {
public:
    int lengthOfLIS(vector<int>& arr) {
        // O(n^2) solution
        // f[i] = length of the LIS that has tail arr[i]
        vector<int> f;
        f.resize(arr.size());
        f[0] = 1;
        for(int i=1;i<arr.size();i++) {
            f[i] = 1;
            for(int j=0;j<i;j++) {
              if(arr[j] < arr[i]) {
                f[i] = max(f[i], f[j] + 1);
              }
            }
        }
        return *max_element(f.begin(), f.end());
    }
};