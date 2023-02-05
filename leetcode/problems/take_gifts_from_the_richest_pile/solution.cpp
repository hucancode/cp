class Solution {
public:
    long long pickGifts(vector<int>& gifts, int k) {
        long long ret = 0;
        priority_queue<int> q;
        for(auto x: gifts) {
            q.push(x);
        }
        while(k--) {
            auto x = q.top();
            q.pop();
            q.push(sqrt(x));
        }
        while(!q.empty()) {
            ret += q.top();
            q.pop();
        }
        return ret;
    }
};