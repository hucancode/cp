class Solution {
public:
    bool canBeEqual(string s1, string s2) {
        vector<char> ac1;
        vector<char> ac2;
        vector<char> bd1;
        vector<char> bd2;
        int n = s1.size();
        for(int i = 0;i<n;i+=2) {
            ac1.push_back(s1[i]);
            ac2.push_back(s2[i]);
        }
        for(int i = 1;i<n;i+=2) {
            bd1.push_back(s1[i]);
            bd2.push_back(s2[i]);
        }
        sort(ac1.begin(),ac1.end());
        sort(ac2.begin(),ac2.end());
        sort(bd1.begin(),bd1.end());
        sort(bd2.begin(),bd2.end());
        return ac1 == ac2 && bd1 == bd2;
    }
};