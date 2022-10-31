class Solution {
public:
    bool same(vector<int>& a, vector<int>& b) {
        int n = a.size();
        for(int i = 0;i<n;i++) {
            if(a[i]!=b[i]) {
                return false;
            }
        }
        return true;
    }
    string oddString(vector<string>& words) {
        vector<int> a;
        vector<int> b;
        int n = words.size();
        int m = words[0].size();
        for(int j = 1;j<m;j++) {
            a.push_back(words[0][j]-words[0][j-1]);
        }
        for(int j = 1;j<m;j++) {
            b.push_back(words[1][j]-words[1][j-1]);
        }
        for(int i = 2;i<n;i++) {
            vector<int> c;
            for(int j = 1;j<m;j++) {
                c.push_back(words[i][j]-words[i][j-1]);
            }
            bool sameac = same(a,c);
            bool samebc = same(b,c);
            if(sameac && !samebc) {
                return words[1];
            }
            if(!sameac && samebc) {
                return words[0];
            }
            if(!sameac && !samebc) {
                return words[i];
            }
        }
        return words[0];
    }
};