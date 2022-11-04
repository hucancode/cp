class Solution {
public:
    string reverseVowels(string s) {
        int n = s.size();
        vector<int> f;
        for(int i = 0;i<n;i++) {
            if(s[i] == 'a' || s[i] == 'e' || s[i] == 'i' || s[i] == 'o' || s[i] == 'u' ||
              s[i] == 'A' || s[i] == 'E' || s[i] == 'I' || s[i] == 'O' || s[i] == 'U') {
                
                f.push_back(i);
            }
        }
        n = f.size();
        for(int i = 0;i<n/2;i++) {
            int j = n-i-1;
            swap(s[f[i]], s[f[j]]);
        }
        return s;
    }
};