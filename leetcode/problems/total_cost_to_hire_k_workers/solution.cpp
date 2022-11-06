class Solution {
public:
    long long totalCost(vector<int>& costs, int k, int candidates) {
        typedef pair<int,int> pi;
        typedef priority_queue<int,vector<int>,greater<int>> min_queue;
        min_queue poolA;
        min_queue poolB;
        int n = costs.size();
        int i = 0;
        for(;i<candidates;i++) {
            //cout<<"pool a push "<<costs[i]<<endl;
            poolA.emplace(costs[i]);
        }
        int j = n-1;
        int j0 = max(i-1, n-1-candidates);
        for(;j>j0;j--) {
            //cout<<"pool b push "<<costs[j]<<endl;
            poolB.emplace(costs[j]);
        }
        long long ret = 0;
        while(k--) {
            if(poolA.empty()) {
                ret += poolB.top();
                poolB.pop();
                continue;
            }
            if(poolB.empty()) {
                ret += poolA.top();
                poolA.pop();
                continue;
            }
            int a = poolA.top();
            int b = poolB.top();
            if(a <= b) {
                ret += a;
                poolA.pop();
                if(i < min(j+1,n)) {
                    poolA.emplace(costs[i]);
                    i++;
                }
                //cout<<"take from pool a "<<a<<endl;
            } else {
                ret += b;
                poolB.pop();
                if(j>=i) {
                    poolB.emplace(costs[j]);
                    j--;
                }
                //cout<<"take from pool b "<<b<<endl;
            }
        }
        return ret;
    }
};