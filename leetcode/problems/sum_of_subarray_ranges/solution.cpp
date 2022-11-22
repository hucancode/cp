class Solution {
public:
    long long subArrayRanges(vector<int>& nums) {
        int n = nums.size();
        vector<int> smallerL(n); // f[i] = how long until we meet an item smaller than nums[i] to the left
        vector<int> smallerR(n); // f[i] = how long until we meet an item smaller than nums[i] to the right
        vector<int> biggerL(n); // f[i] = how long until we meet an item bigger than nums[i] to the left
        vector<int> biggerR(n); // f[i] = how long until we meet an item bigger than nums[i] to the right
        stack<int> st;
        for(int i = 0;i<n;i++) {
            while(!st.empty() && nums[i] < nums[st.top()]) {
                st.pop();
            }
            smallerL[i] = st.empty()?(i+1):(i - st.top());
            st.push(i);
        }
        while(!st.empty()) st.pop();
        for(int i = n-1;i>=0;i--) {
            while(!st.empty() && nums[i] <= nums[st.top()]) {
                st.pop();
            }
            smallerR[i] = st.empty()?(n-i):(st.top() - i);
            st.push(i);
        }
        while(!st.empty()) st.pop();
        for(int i = 0;i<n;i++) {
            while(!st.empty() && nums[i] > nums[st.top()]) {
                st.pop();
            }
            biggerL[i] = st.empty()?(i+1):(i - st.top());
            st.push(i);
        }
        while(!st.empty()) st.pop();
        for(int i = n-1;i>=0;i--) {
            while(!st.empty() && nums[i] >= nums[st.top()]) {
                st.pop();
            }
            biggerR[i] = st.empty()?(n-i):(st.top() - i);
            st.push(i);
        }
        typedef long long ll;
        ll sumMax = 0;
        ll sumMin = 0;
        for(int i = 0;i<n;i++) {
            sumMax += (ll)nums[i] * biggerL[i] * biggerR[i];
            sumMin += (ll)nums[i] * smallerL[i] * smallerR[i];
        }
        return sumMax - sumMin;
    }
};