class Solution {
public:
    int minKBitFlips(vector<int>& nums, int k) {
        int n = nums.size();
        int flipCount = 0;
        queue<int> flipHistory;
        for(int i = 0;i<n;i++) {
            bool v = nums[i];
            if(!flipHistory.empty() && flipHistory.front() < i) {
                flipHistory.pop();
            }
            if(flipHistory.size()%2) {
                v = !v;
            }
            bool shouldFlip = v == 0;
            bool canFlip = i+k-1 < n;
            if(!shouldFlip) {
                continue;
            }
            if(!canFlip) {
                return -1;
            }
            flipCount++;
            flipHistory.push(i+k-1);
        }
        return flipCount;
    }
};