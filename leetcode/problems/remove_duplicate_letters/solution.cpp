class Solution {
public:
    string removeDuplicateLetters(string s) {
        map<char,int> count;
        map<char,bool> vis;
        for(auto c:s) count[c]++;
        string ret;
        for(auto c:s) {
            count[c]--;
            if(vis[c]) continue;
            while(!ret.empty() && c < ret.back() && count[ret.back()] > 0) {
                vis[ret.back()] = false;
                ret.pop_back();
            }
            ret.push_back(c);
            vis[c] = true;
        }
        return ret;
    }
};