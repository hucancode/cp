class Solution {
public:
    int romanToInt(string s) {
        vector<pair<string, int>> dict = {
            {"CM", 900},
            {"M", 1000},
            {"CD", 400},
            {"D", 500},
            {"XC", 90},
            {"C", 100},
            {"XL", 40},
            {"L", 50},
            {"IX", 9},
            {"X", 10},
            {"IV", 4},
            {"V", 5},
            {"I", 1},
        };
        for(auto entry: dict) {
            int n = entry.first.size();
            if(s.substr(0, n) == entry.first) {
                return entry.second+ romanToInt(s.substr(n));
            }
        }
        return 0;
    }
};