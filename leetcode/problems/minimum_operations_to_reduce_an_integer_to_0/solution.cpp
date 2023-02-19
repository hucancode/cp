class Solution {
public:
    int minOperations(int n) {
        vector<bool> bits;
        while(n != 0) {
            bits.push_back(n%2);
            n/=2;
        }
        bits.push_back(0);
        bits.push_back(0);
        int ret = 0;
        int streak = 0;
        for(auto x: bits) {
            if(x == 0) {
                if(streak > 1) {
                    streak = 1;
                    ret++;
                } else if(streak == 1) {
                    streak = 0;
                    ret++;
                }
            } else if(x == 1) {
                streak++;
            }
        }
        return ret;
    }
};