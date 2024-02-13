#include <ranges>
#include <cctype>
class Solution {
public:
    bool isPalindrome(string s) {
        auto it = s | 
            views::filter([](char c) {
                return isalnum(c);
            }) | 
            views::transform([](char c) {
                return tolower(c);
            });
        return ranges::equal(it, it | views::reverse);
    }
};