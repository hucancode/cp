class Solution {
public:
    string addBinary(string a, string b) {
        reverse(a.begin(), a.end());
        reverse(b.begin(), b.end());
        int remainder = 0;
        string c = "";
        auto i = a.begin();
        auto j = b.begin();
        while(i != a.end() || j != b.end() || remainder != 0) {
            auto x = (i == a.end())?0:(*i - '0');
            auto y = (j == b.end())?0:(*j - '0');
            auto z = x+y+remainder;
            c += (z%2) + '0';
            remainder = z/2;
            if(i!= a.end()) i++;
            if(j!= b.end()) j++;
        }
        reverse(c.begin(), c.end());
        return c;
    }
};