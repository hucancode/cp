class Solution {
public:
    int maxLength(vector<string>& arr) {
        const int n = arr.size();
        vector<int> mask(n, 0);
        for(int i = 0;i<n;i++) {
            for(const auto c: arr[i]) {
                int k = c - 'a';
                if(mask[i] & (1<<k)) {
                    // self colliding
                    mask[i] = 0;
                    break;
                }
                mask[i] |= 1<<k;
            }
        }
        mask.erase(remove(mask.begin(), mask.end(), 0), mask.end());
        return dfs(mask);
    }
    int dfs(vector<int>& mask) {
        const int n = mask.size();
        int ret = 0;
        stack<int> st;
        for(int i = 0;i<n;i++) {
            st.push(i);
        }
        int currentMask = 0;
        while(!st.empty()) {
            int i = st.top();
            st.pop();
            if(i < 0) {
                // done traverse i, undo change in i and go next
                currentMask &= ~mask[-i];
                continue;
            }
            bool collide = mask[i] & currentMask;
            if(collide) {
                continue;
            }
            currentMask |= mask[i];
            ret = max(ret, __builtin_popcount(currentMask));
            st.push(-i); // -i means undo change in i
            for(int j = i+1;j<n;j++) {
                st.push(j);
            }
        }
        return ret;
    }
};