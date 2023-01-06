class Solution {
public:
    long long maximumBeauty(vector<int>& flowers, long long newFlowers, int target, int full, int partial) {
        typedef long long ll;
        typedef vector<ll> vl;
        sort(flowers.rbegin(), flowers.rend());
        int n = flowers.size();
        vl left(n); // left[i] cost to make garden[0~i] full
        vl right(n); // right[i] cost to make garden[i~n) partial
        left[0] = max(0, target - flowers[0]);
        right[n-1] = 0;
        for(int i = 1;i<n;i++) {
            left[i] = left[i-1] + max(0, target - flowers[i]);
        }
        for(int i = n-2;i>=0;i--) {
            if(flowers[i] >= target) {
                break;
            }
            right[i] = right[i+1] + (flowers[i] - flowers[i+1]) * (n - i - 1);
        }
        ll ret = 0;
        int i = -1;
        int j = 0;
        while(j < n && flowers[j] >= target) {
            j++;
        }
        while(i<n || j<n) {
            auto costL = i<0?0L:left[i];
            auto costR = j<n?right[j]:0L;
            auto remainder = newFlowers - costL - costR;
            if(remainder < 0) {
                if(j<n) {
                    j++;
                } else {
                    i++;
                }
                continue;
            }
            auto scoreL = (i+1)*(ll)full;
            auto scoreR = 0L;
            if(j<n) {
                ll extra = remainder/(n-j);
                ll k = flowers[j] + extra;
                k = min(k, (ll)target-1);
                scoreR = k*partial;
            }
            ret = max(ret, scoreL + scoreR);
            i++;
            j = max(j,i+1); 
        }
        return ret;
    }
};