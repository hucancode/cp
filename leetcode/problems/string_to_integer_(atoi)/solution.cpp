#define MAX_INT 2147483647L
#define MIN_INT -2147483648L

class Solution {
public:
    int myAtoi(string s) {
        bool negative = false;
        s = normalize(s);
        cout<<"normalize = "<<s<<endl;
        int i = 0;
        if(s[0] == '-') {
            negative = true;
            s.erase(s.begin());
        }
        if(s[0] == '+') {
            s.erase(s.begin());
        }
        long ret = 0;
        long factor = 1;
        for(i = s.size() - 1;i>=0;i--) {
            int digit = (s[i] - '0');
            ret += digit*factor;
            if(factor > abs(MIN_INT) || ret > abs(MIN_INT)) {
                ret = abs(MIN_INT);
                break;
            }
            factor *= 10;
            
        }
        if(negative) {
            ret = -ret;
        }
        cout<<"long parsed = "<<ret<<endl;
        ret = max(MIN_INT, min(ret, MAX_INT));
        return (int)ret;
    }
    string normalize(string s) {
        string ret = "";
        int i = 0;
        while(i < s.size() && s[i] == ' ') {
            i++;
        }
        if(i < s.size() && (s[i] == '+' || s[i] == '-')) {
            ret += s[i];
            i++;
        }
        while(i < s.size() && s[i] == '0') {
            i++;
        }
        for(;i<s.size();i++) {
            if(s[i] <= '9' && s[i] >= '0') {
                ret += s[i];
            } else {
                break;
            }
        }
        return ret;
    }
};