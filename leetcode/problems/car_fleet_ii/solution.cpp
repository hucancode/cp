class Solution {
public:
    double collideTime(vector<int>& a, vector<int>& b) {
        if(b[1] >= a[1]) {
            return -1;
        }
        return ((double)b[0]-a[0])/(a[1]-b[1]);
    }
    vector<double> getCollisionTimes(vector<vector<int>>& cars) {
        int n = cars.size();
        // observation:
        // all cars behind A will not affect A's speed, therefore will not affect A's collision time
        // - calculate from right to left
        // consider ordered cars A - B - C
        // - if BC collide before AB collide, time(AB) > time(AC)
        //  + in other words, B will not affect A's collision time
        // - if AB collide before BC collide, that's basic calculation between AB only
        //  + and that result will not affect B's collision time so we don't need to recalculate anything
        vector<double> ret(n, -1.0);
        vector<int> st;
        for(int a = n-1;a>=0;a--) {
            while(!st.empty()) {
                int b = st.back();
                double tab = collideTime(cars[a], cars[b]);
                double tbc = ret[b];
                if(tab == -1.0 || (tbc != -1.0 && tab > tbc)) {
                    st.pop_back();
                } else {
                    ret[a] = tab;
                    break;
                }
            }
            st.push_back(a);
        }
        return ret;
    }
};