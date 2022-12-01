class Solution {
public:
    bool isVowel(char x) {
        string s = "aeiouAEIOU";
        for(auto c: s) {
            if(c == x) {
                return true;
            }
        }
        return false;
    }
    bool halvesAreAlike(string s) {
        int n = s.size();
        int a = 0, b = 0;
        for(int i = 0;i<n/2;i++) {
            int j = i+n/2;
            if(isVowel(s[i])) {
                a++;
            }
            if(isVowel(s[j])) {
                b++;
            }
        }
        return a == b;
    }
};