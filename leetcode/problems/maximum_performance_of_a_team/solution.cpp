class Solution {
public:
    int maxPerformance(int n, vector<int>& speed, vector<int>& efficiency, int k) {
        vector<pair<int,int>> a;
        a.reserve(n);
        for(int i = 0;i<n;i++) {
            a.emplace_back(efficiency[i], speed[i]);
        }
        sort(a.rbegin(), a.rend());
        const int INF = 1e9+7;
        long long ret = 0;
        long long bestTotalSpd = 0;
        priority_queue<int, vector<int>, greater<int>> q;
        for(int i = 0;i<n;i++) {
            int worstEff = a[i].first;
            int spd = a[i].second;
            q.push(spd);
            bestTotalSpd += spd;
            if(q.size() > k) {
                bestTotalSpd -= q.top();
                q.pop();
            }
            long long score = bestTotalSpd*worstEff;
            ret = max(ret, score);
            //cout<<"at "<<i<<" bestTotalSpd = "<<bestTotalSpd<<" worstEff = "<<worstEff<<"use "<<q.size()<<" men "<<" score = "<<score<<endl;
        }
        return ret%INF;
    }
};