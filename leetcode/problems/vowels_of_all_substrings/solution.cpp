class Solution {
public:
    long long countVowels(string word) {
        int n = word.size();
        long long ret = 0;
        for(int i = 0;i<n;i++) {
            char c = word[i];
            if(c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
                int l = i+1;// there are i+1 ways to pick left part
                int r = n-i;// there are n-i ways to pick right part
                ret += (long long)l*r;
            }
        }
        return ret;
    }
};