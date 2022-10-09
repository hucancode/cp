class Solution {
public:
    int lengthOfLIS(vector<int>& arr) {
        // O(nlogn) solution
        // tail[i] = smallest tail for LIS length i
        vector<int> tail;
        tail.reserve(arr.size());
        tail.push_back(arr[0]);
        for(int i=1;i<arr.size();i++) {
            auto it = lower_bound(tail.begin(), tail.end(), arr[i]);
            if(it == tail.end()) {
              // if this new arr[i] bigger than every LIS tail ever found, we have new LIS
              tail.push_back(arr[i]);
            } else {
              // if this new arr[i] fit in existing LIS, update LIS
              *it = arr[i];
            }
        }
        return tail.size();
    }
};