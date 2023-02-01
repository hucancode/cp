class Solution {
public:
    bool dup(int x, string str) {
        int m = str.size()/x;
        for(int i = 0;i<x;i++) {
            for(int j = 1;j<m;j++) {
                if(str[j*x+i] != str[i]) {
                    return false;
                }
            }
        }
        return true;
    }
    string gcdOfStrings(string str1, string str2) {
        int k = gcd(str1.size(), str2.size());
        int i = 1;
        cout<<k<<endl;
        while(k > 0) {
            if(dup(k, str1) && dup(k, str2)) {
                if(str1.substr(0,k) == str2.substr(0,k)) {
                    return str1.substr(0, k);
                }
            }
            k/=++i;
        }
        return "";
    }
};