class Solution {
public:
    string firstPalindrome(vector<string>& words) {
        auto i = find_if(words.begin(), words.end(), [](string& s){
            return equal(s.cbegin(), s.cbegin() + s.size() / 2, s.crbegin());
        });
        return (i != words.end()) ? *i : "";
    }
};