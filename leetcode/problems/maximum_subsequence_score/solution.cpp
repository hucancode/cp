class Solution {
public:
    long long maxScore(vector<int>& nums1, vector<int>& nums2, int k) {
        int n = nums1.size();
        long long score = 0;
        long long sum = 0;
        
        vector<pair<int,int>> f; f.reserve(n);
        for(int i = 0;i<n;i++) {
            f.emplace_back(nums2[i], i);
        }
        sort(f.rbegin(), f.rend());
        int x = f[0].first;
        typedef pair<int,int> state;
        priority_queue<state, vector<state>, greater<state>> pq;
        for(int i = 0;i<k;i++) {
            auto j = f[i].second;
            sum += nums1[j];
            pq.emplace(nums1[j], j);
            x = f[i].first;
        }
        score = sum * x;
        for(int i = k;i<n;i++) {
            auto j = f[i].second;
            sum += nums1[j];
            pq.emplace(nums1[j], j);
            while(pq.size() > k) {
                sum -= nums1[pq.top().second];
                pq.pop();
            }
            x = f[i].first;
            score = max(score, sum*x);
        }
        return score;
    }
};