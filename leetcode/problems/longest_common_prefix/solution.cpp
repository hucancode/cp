class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        int i = 0;
        while(i<strs[0].size()) {
            for(int j = 1;j<strs.size();j++) {
                if(strs[j].size() <= i) {
                    return strs[0].substr(0, i);
                }
                if(strs[j][i] != strs[0][i]) {
                    return strs[0].substr(0, i);
                }
            }
            i++;
        }
        return strs[0].substr(0, i);
    }
};