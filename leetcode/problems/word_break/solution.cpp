class Solution {
public:
    bool exist(string s, vector<string>& wordDict) {
        return binary_search(wordDict.begin(), wordDict.end(), s);
    }
    bool wordBreak(string s, vector<string>& wordDict) {
        sort(wordDict.begin(), wordDict.end());
        int n = s.size();
        vector<int> wordIndices;
        for(int i = 0;i<n;i++) {
            string word = s.substr(0, i+1);
            if(exist(word, wordDict)) {
                wordIndices.push_back(i);
                continue;
            }
            for(const auto j:wordIndices) {
                word = s.substr(j+1, i-j);
                if(exist(word, wordDict)) {
                    wordIndices.push_back(i);
                    break;
                }
            }
        }
        if(wordIndices.empty()) {
            return false;
        }
        return *wordIndices.rbegin() == n-1;
    }
};