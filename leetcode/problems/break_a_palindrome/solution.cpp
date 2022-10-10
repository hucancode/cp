class Solution {
public:
    string breakPalindrome(string palindrome) {
        int n2 = palindrome.size();
        if(n2 == 1) {
            return "";
        }
        if(n2%2 == 1) {
            n2--;
        }
        int n = n2/2;
        for(int i = 0;i<n;i++) {
            if(palindrome[i] != 'a') {
                palindrome[i] = 'a';
                return palindrome;
            }
        }
        palindrome[palindrome.size() - 1] = 'b';
        return palindrome;
    }
};