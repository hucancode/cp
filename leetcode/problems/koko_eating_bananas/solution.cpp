class Solution {
public:
    bool check(vector<int>& piles, int h, int k) {
        for(auto x: piles) {
            h -= x/k;
            if(x%k != 0) h--;
            if(h < 0) {
                return false;
            }
        }
        return true;
    }
    int minEatingSpeed(vector<int>& piles, int h) {
        int l = 1, r = *max_element(piles.begin(), piles.end());
        while(l < r) {
            int m = (l+r)/2;
            bool good = check(piles, h, m);
            if(good) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
};