class Solution {
public:
    int jobScheduling(vector<int>& startTime, vector<int>& endTime, vector<int>& profit) {
        int n = startTime.size();
        vector<int> ref(n);
        for(int i = 0;i<n;i++) {
            ref[i] = i;
        }
        vector<int> f(n+1, 0);
        auto jobCmp = [&](const int& a, const int& b)
        {
            return startTime[a] < startTime[b];
        };
        auto timeCmp = [&](const int& i, const int& time)
        {
            return startTime[i] < time;
        };
        sort(ref.begin(), ref.end(), jobCmp);
        for(int i = n-1;i>=0;i--) {
            auto index = ref[i];
            auto p = profit[index];
            auto start = startTime[index];
            auto end = endTime[index];
            //cout<<"check job ["<<start<<"-"<<end<<"], profit = "<<p<<endl;
            auto j = distance(ref.begin(), lower_bound(ref.begin(), ref.end(), end, timeCmp));
            //cout<<"find job with start time >= "<<end<<", returns index "<<j<<endl;
            if(j < n) {
                p += f[j];
            }
            f[i] = max(p, f[i+1]);
            //cout<<"profit ["<<start<<"-"<<end<<"] = f["<<i<<"]="<<f[i]<<endl;
        }
        return f[0];
    }
};