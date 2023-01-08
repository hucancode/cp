class Solution {
public:
    long long maxKelements(vector<int>& nums, int k) {
        priority_queue<int> q;
        for(auto x:nums) {
            q.push(x);
        }
        long long ret = 0;
        while(k--) {
            auto x = q.top();
            q.pop();
            q.push(ceil(x/3.0));
            ret += x;
        }
        return ret;
    }
};