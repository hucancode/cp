class Solution {
public:
    vector<int> goodIndices(vector<int>& nums, int k) {
        int n = nums.size();
        vector<bool> f(n);
        vector<bool> g(n);
        deque<int> q;
        for(int i = 0;i<n;i++) {
            if(!q.empty() && i - q.front() >= k) {
                q.pop_front();
            }
            while(!q.empty() && nums[q.back()] < nums[i]) {
                q.pop_back();
            }
            q.push_back(i);
            f[i] = q.size() == k;
        }
        q.clear();
        for(int i = n-1;i>=0;i--) {
            if(!q.empty() && q.front() - i >= k) {
                q.pop_front();
            }
            while(!q.empty() && nums[i] > nums[q.back()]) {
                q.pop_back();
            }
            q.push_back(i);
            g[i] = q.size() == k;
        }
        // cout<<"f: ";
        // for(const auto &x: f) {
        //     cout<<" "<<x;
        // }
        // cout<<endl;
        // cout<<"g: ";
        // for(const auto &x:g) {
        //     cout<<" "<<x;
        // }
        vector<int> ret;
        for(int i = 1;i<n-1;i++) {
            if(f[i-1] && g[i+1]) {
                ret.push_back(i);
            }
        }
        return ret;
    }
};