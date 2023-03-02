class Solution {
public:
    int compress(vector<char>& chars) {
        if(chars.size() <= 1) {
            return chars.size();
        }
        auto i = chars.begin();
        auto j = i+1;
        while(i != chars.end()) {
            if(j != chars.end() && *i == *j) {
                j++;
                continue;
            }
            int len = distance(i, j);
            if(len <= 1) {
                i = j++;
                continue;
            }
            i++;
            int k = 1000;
            while(k > len) k /= 10;
            while(k > 0) {
                char digit = len/k + '0';
                *i = digit;
                len = len%k;
                k /= 10;
                i++;
            }
            j = chars.erase(i,j);
            i = j++;
        }
        return chars.size();
    }
};