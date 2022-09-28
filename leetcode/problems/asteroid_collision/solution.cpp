class Solution {
public:
    vector<int> asteroidCollision(vector<int>& asteroids) {
        stack<int> st;
        vector<int> ret;
        for(const auto& x: asteroids) {
            if(x > 0) {
                st.push(x);
                continue;
            }
            while(!st.empty() && st.top() < abs(x)) {
                st.pop();
            }
            if(st.empty()) {
                ret.push_back(x);
            } else if(abs(x) == st.top()) {
                st.pop();
            }
        }
        int n = ret.size();
        while(!st.empty()) {
            ret.insert(ret.begin()+n,st.top());
            st.pop();
        }
        return ret;
    }
};