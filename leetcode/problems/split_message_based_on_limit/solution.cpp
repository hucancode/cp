class Solution {
public:
    void addFiller(vector<string>& arr) {
        int m = arr.size();
        for(int i = 0;i<m;i++) {
            ostringstream ss;
            ss<<'<'<<i+1<<'/'<<m<<'>';
            arr[i] += ss.str();
        }
    }
    vector<string> splitMessage(string message, int limit) {
        // arr len contains 1 digit
        // -> must have filler <a/b> 5 char
        // arr len contains 2 digit
        // -> must have filler <a/bc> 6 char, <ab/cd> 7 char
        // arr len contains 3 digit 
        // -> must have filler <a/bcd> 7 char, <ab/cde> 8 char, <abc/def> 9 char
        // can arr len be <= 9? 9*(limit - 5) >= n
        // can arr len be <= 99? (limit - 6)*9 + (limit - 7)*90 >= n
        // can arr len be <= 999? (limit - 7)*9 + (limit - 8)*90 + (limit - 9)*900 >= n
        // can arr len be <= 9999? (limit - 8)*9 + (limit - 9)*90 + (limit - 10)*900 + (limit - 11)*9000 >= n
        int n = message.size();
        int i = 0;
        vector<string> ret;
        int cap = 9*(limit - 5); // split into 9 parts, how many char can we handle?
        if(cap >= n) {
            while(i<n) {
                int len = limit - 5;
                ret.push_back(message.substr(i, len));
                i+= len;
            }
            addFiller(ret);
            return ret; 
        }
        cap = (limit - 6)*9 + (limit - 7)*90;
        if(cap >= n) {
            while(i<n) {
                int flen = ret.size()<9?6:7;
                int len = limit - flen;
                ret.push_back(message.substr(i, len));
                i+= len;
            }
            addFiller(ret);
            return ret; 
        }
        cap = (limit - 7)*9 + (limit - 8)*90 + (limit - 9)*900;
        if(cap >= n) {
            while(i<n) {
                int flen = 9;
                if(ret.size() < 9) {
                    flen = 7;
                } else if(ret.size() < 99) {
                    flen = 8;
                }
                int len = limit - flen;
                ret.push_back(message.substr(i, len));
                i+= len;
            }
            addFiller(ret);
            return ret; 
        }
        cap = (limit - 8)*9 + (limit - 9)*90 + (limit - 10)*900 + (limit - 11)*9000;
        if(cap >= n) {
            while(i<n) {
                int flen = 11;
                if(ret.size() < 9) {
                    flen = 8;
                } else if(ret.size() < 99) {
                    flen = 9;
                } else if(ret.size() < 999) {
                    flen = 10;
                }
                int len = limit - flen;
                ret.push_back(message.substr(i, len));
                i+= len;
            }
            addFiller(ret);
            return ret;
        }
        return ret;
    }
};