class Solution {
public:
    bool isPalindrome(string& s) {
        int n = s.size();
        for(int i = 0;i<n/2;i++) {
            if(s[i] != s[n-i-1]) {
                return false;
            }
        }
        return true;
    }
    void traverse(vector<vector<string>>& ret, vector<string> prefix, string partition) {
        if(partition.empty()) {
            ret.push_back(prefix);
            return;
        }
        int n = partition.size();
        for(int i = 1;i<=n;i++) {
            string left = partition.substr(0,i);
            if(isPalindrome(left)) {
                string right = partition.substr(i);
                prefix.push_back(left);
                traverse(ret, prefix, right);
                prefix.pop_back();
            }
        }

    }
    vector<vector<string>> partition(string s) {
        vector<vector<string>> ret;
        vector<string> prefix;
        traverse(ret, prefix, s);
        return ret;
    }
};