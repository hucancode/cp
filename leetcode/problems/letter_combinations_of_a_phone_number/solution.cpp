class Solution {
public:
    vector<string> letterCombinations(string digits) {
        vector<string> ret;
        if(digits.empty()) {
            return ret;
        }
        vector<string> dict = {
            "abc", "def", 
            "ghi", "jkl", "mno", 
            "pqrs", "tuv", "wxyz"
        };
        int n = digits.size();
        stack<pair<int, int>> q;
        int i = 0;
        int j = 0;
        int d = digits[i] - '2';
        for(;j<dict[d].size();j++) {
            q.emplace(i, j);
        }
        string ans = "";
        while(!q.empty()) {
            tie(i, j) = q.top();
            q.pop();
            if(i < 0) {
                ans.pop_back();
                continue;
            }
            d = digits[i] - '2';
            ans.push_back(dict[d][j]);
            if(i == n - 1) {
                ret.push_back(ans);
                ans.pop_back();
                continue;
            }
            q.emplace(-1, 0);
            i++;
            d = digits[i] - '2';
            for(j = 0;j<dict[d].size();j++) {
                q.emplace(i, j);
            }
        }
        return ret;
    }
};