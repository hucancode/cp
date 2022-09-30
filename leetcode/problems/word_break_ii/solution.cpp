class Solution {
public:
    bool exist(string s, vector<string>& wordDict) {
        return binary_search(wordDict.begin(), wordDict.end(), s);
    }
    vector<string> trace(string s, vector<string>& wordDict, vector<int> pivots) {
        // cout<<"trace "<<s<<", pivot = ";
        // for(const auto x: pivots) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        vector<string> ret;
        if(exist(s, wordDict)) {
            ret.push_back(s);
        }
        int n = s.size();
        if(pivots.empty()) {
            return ret;
        }
        if(*pivots.rbegin() != n-1) {
            return ret;
        }
        pivots.pop_back();
        for(int i = 0;i<pivots.size();i++) {
            int k = pivots[i]+1;
            string word = s.substr(k);
            if(!exist(word, wordDict)) {
                continue;
            }
            vector<int> nextPivots(pivots.begin(), pivots.begin()+i+1);
            auto children = trace(s.substr(0, k), wordDict, nextPivots);
            for(const auto& x: children) {
                ret.push_back(x+" "+word);
            }
        }
        // cout<<"return ";
        // for(const auto x: ret) {
        //     cout<<x<<" ";
        // }
        // cout<<endl;
        return ret;
    }
    vector<string> wordBreak(string s, vector<string>& wordDict) {
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
        return trace(s, wordDict, wordIndices);
    }
};