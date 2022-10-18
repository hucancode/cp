class Solution {
public:
    string int2str(int k) {
        string ret;
        while(k != 0) {
            int i = k%10;
            k /= 10;
            ret.push_back(i+'0');
        }
        reverse(ret.begin(), ret.end());
        return ret;
    }
    string countAndSay(int n) {
        if(n == 1) {
            return "1";
        }
        string stem = countAndSay(n-1);
        char current = stem[0];
        int count = 0;
        string ret;
        for(const auto x: stem) {
            if(x == current) {
                count++;
            } else {
                ret += int2str(count);
                ret.push_back(current);
                current = x;
                count = 1;
            }
        }
        ret += int2str(count);
        ret.push_back(current);
        return ret;
    }
};