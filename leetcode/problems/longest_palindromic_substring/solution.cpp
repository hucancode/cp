class Solution {
public:
    bool palindrome(string::iterator i, string::iterator j) {
        while(i < j) {
            if(*i != *j) {
                return false;
            }
            i++;
            j--;
        }
        return true;
    }
    string longestPalindrome(string s) {
        int n = s.size();
        string ret = s.substr(0,1);
        for(int i = 0;i<n-1;i++) {
            auto minLen = ret.size()/2+1;
            for(int len = minLen;len < n - i;len++) {
                //cout<<"check from s["<<i<<"] "<<s[i]<<" with length "<<len<<endl;
                int j = i + (len - 1)*2;
                if(j >= n) {
                    break;
                }
                if(palindrome(s.begin() + i, s.begin() + j)) {
                    ret = s.substr(i, j - i + 1);
                }
                j += 1;
                if(j >= n) {
                    break;
                }
                if(palindrome(s.begin() + i, s.begin() + j)) {
                    ret = s.substr(i, j - i + 1);
                }
            }
        }
        return ret;
    }
};