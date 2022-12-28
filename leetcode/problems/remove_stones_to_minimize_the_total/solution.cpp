class Solution {
public:
    int minStoneSum(vector<int>& piles, int k) {
        priority_queue<int> q;
        for(auto x: piles) {
            q.push(x);
        }
        while(k--) {
            auto x = q.top();
            q.pop();
            x -= x/2;
            q.push(x);
        }
        int ret = 0;
        while(!q.empty()) {
            ret += q.top();
            q.pop();
        }
        return ret;
    }
};