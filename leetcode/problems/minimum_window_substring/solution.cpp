class Solution {
public:
    string minWindow(string s, string t) {
        int m = s.size();
        int n = t.size();
        int k = 123;
        int target[k];
        fill(target, target + k, 0);
        int count[k];
        fill(count, count + k, 0);
        for(int i = 0;i<n;i++) {
            target[t[i]]++;
        }
        int l = 0;
        int r = 0;
        int bestIndex = 0;
        int bestLen = 0;
        count[s[0]]++;
        while(l <= m - n) {
            bool ok = true;
            //cout<<"checking... l="<<l<<", r="<<r<<endl;
            int len = r-l+1;
            if(len < n) {
                ok = false;
            } else {
                for(int i = 0;i<k;i++) {
                    if(target[i] > count[i]) {
                        //cout<<"failed at "<<(char)i<<endl;
                        ok = false;
                        break;
                    }
                }
            }
            if(ok) {
                if(bestLen == 0 || len < bestLen) {
                    bestLen = len;
                    bestIndex = l;
                }
                //cout<<"passed! bestLen = "<<bestLen<<" bestIndex = "<<bestIndex<<endl;
                count[s[l]]--;
                l++;
            } else {
                r++;
                if(r >= m) {
                    break;
                }
                count[s[r]]++;
            }
        }
        if(bestLen == 0) {
            return "";
        }
        return s.substr(bestIndex, bestLen);
    }
};