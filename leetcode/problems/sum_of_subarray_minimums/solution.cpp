class Solution {
public:
    int sumSubarrayMins(vector<int>& arr) {
        const int INF = 1e9+7;
        int n = arr.size();
        vector<int> weightLeft(n); // weight[i] = how long until we meet an item smaller than arr[i] to the left
        vector<int> weightRight(n); // weight[i] = how long until we meet an item smaller than arr[i] to the right
        stack<int> st;
        for(int i = 0;i<n;i++) {
            while(!st.empty() && arr[i] < arr[st.top()]) {
                st.pop();
            }
            weightLeft[i] = st.empty()?(i+1):(i - st.top());
            st.push(i);
        }
        while(!st.empty()) st.pop();
        for(int i = n-1;i>=0;i--) {
            while(!st.empty() && arr[i] <= arr[st.top()]) {
                st.pop();
            }
            weightRight[i] = st.empty()?(n-i):(st.top() - i);
            st.push(i);
        }
        // cout<<"left = ";
        // for(auto x: weightLeft) {
        //     cout<<x<<" ";
        // }
        // cout<<"\nright = ";
        // for(auto x: weightRight) {
        //     cout<<x<<" ";
        // }
        long long ret = 0;
        for(int i = 0;i<n;i++) {
            ret += (long long)arr[i]*weightLeft[i]*weightRight[i]%INF;
            ret %= INF;
        }
        return ret;
    }
};