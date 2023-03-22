class Solution {
public:
    double new21Game(int n, int k, int maxPts) {
        if(k == 0) {
            return 1;
        }
        vector<double> f(k+maxPts, 1.0);
        // f[i] = probability to reach score i
        // f[0] = 1
        // f[1] = f[0]/m;
        // f[2] = f[1]/m + f[0]/m;
        // f[3] = f[2]/m + f[1]/m + f[0]/m;
        // f[i] = (sum for each j in 1..m (f[j]))/m; skip if j>k
        double sum = 1.0;
        for(int i = 1;i<f.size();i++) {
            int j = i-maxPts-1;
            if(j >= 0) {
                sum -= f[j];
            }
            f[i] = sum/maxPts;
            if(i < k) {
                sum += f[i];
            }
        }
        double ret = 1.0;
        for(int i = n+1;i<f.size();i++) {
            ret -= f[i];
        }
        return ret;
    }
};