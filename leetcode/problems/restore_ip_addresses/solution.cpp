class Solution {
public:
    void traverse(int i, int k, int dot, string ip, string& s, vector<string>& ret) {
        //cout<<"traverse k = "<<k<<" i = "<<i<<" ip = "<<ip<<endl;
        if(dot > 3) {
            return;
        }
        int n = s.size();
        if(i == n) {
            if(dot == 3) {
                ret.push_back(ip);
            }
            return;
        }
        auto d = s[i] - '0';
        if(ip == "0" || 
            (ip.size() > 2 && ip.substr(ip.size()-2, 2) == ".0")) {
            return;
        }
        k = k*10 + d;
        if(k > 255) {
            return;
        }
        ip += s[i];
        if(i < n-1) {
            traverse(i+1, 0, dot+1, ip+".", s, ret);
        }
        traverse(i+1, k, dot, ip, s, ret);
    }
    vector<string> restoreIpAddresses(string s) {
        vector<string> ret;
        traverse(0, 0, 0, "", s, ret);
        return ret;
    }
};