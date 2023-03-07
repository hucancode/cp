class Solution {
public:
    long long check(vector<int>& time, int totalTrips, long long k) {
        int n = time.size();
        for(int i = 0;i<n;i++) {
            auto trip = k/time[i];
            totalTrips -= trip;
            if(totalTrips <= 0) {
                return true;
            }
        }
        return false;
    }
    long long minimumTime(vector<int>& time, int totalTrips) {
        int x = *min_element(time.begin(), time.end());
        long long l = x, r = x*(long long)totalTrips;
        while(l < r) {
            //cout<<"search "<<l<<"-"<<r<<endl;
            auto m = (l+r)/2;
            auto good = check(time, totalTrips, m);
            if(good) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
};