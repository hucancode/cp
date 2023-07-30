class Solution {
public:
    string concat(string& a, string& b) {
        int n = a.size();
        int m = b.size();
        for(int i = 0;i<n;i++) {
            int len = min(n-i,m);
            if(a.compare(i, len, b, 0, len) != 0) continue;
            if(n-i > m) return a;
            return a.substr(0, i) + b;
        }
        return a + b;
    }
    string minimumString(string a, string b, string c) {
        vector<string> input = {a, b, c};
        sort(input.begin(), input.end());
        string ret = "";
        do {
            auto ab = concat(input[0], input[1]);
            auto abc = concat(ab, input[2]);
            if(ret.empty() || 
                abc.size() < ret.size() || 
                (abc.size() == ret.size() && abc.compare(ret) < 0)) {
                ret = abc;
            }
        } while(next_permutation(input.begin(), input.end()));
        return ret;
    }
};