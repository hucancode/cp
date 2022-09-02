class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int n = s.size();
        if(n == 0) {
            return 0;
        }
        int i = 0;
        int j = 1;
        int ret = 1;
        bool exist[256];
        fill(exist, exist+256, false);
        exist[s[0]] = true;
        while(i < n) {
            while(j < n) {
                if(exist[s[j]]) {
                    break;
                } else {
                    exist[s[j]] = true;
                    j++;
                }
            }
            ret = max(ret, j - i);
            exist[s[i]] = false;
            i++;
        }
        return ret;
    }
};