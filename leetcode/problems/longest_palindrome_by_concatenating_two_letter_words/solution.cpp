class Solution {
public:
    int longestPalindrome(vector<string>& words) {
        int ret = 0;
        map<string, int> counts;
        for(const auto& x: words) {
            if(counts.find(x) == counts.end()) {
                counts[x] = 0;
            }
            counts[x]++;
        }
        for(const auto& x: words) {
            string rev(x.rbegin(), x.rend());
            //cout<<"word = "<<x<<" rev = "<<rev<<endl;
            if(counts.find(rev) == counts.end()) {
                continue;
            }
            int c = min(counts[x], counts[rev]);
            if(rev == x) {
                c /= 2;
            }
            //cout<<"found "<<c<<" pairs"<<endl;
            counts[x] -= c;
            counts[rev] -= c;
            ret += 4*c;
        }
        for(const auto& x: words) {
            if(counts[x] == 0) {
                continue;
            }
            if(x[0] != x[1]) {
                continue;
            }
            ret += 2;
            break;
        }
        return ret;
    }
};