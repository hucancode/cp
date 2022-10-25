class Solution {
public:
    inline bool isConcat(const string& word, const vector<string>& words) {
        //cout<<"isConcat "<<word<<endl;
        int n = word.size();
        if(n == 1) {
            return false;
        }
        int minLen = words.begin()->size();
        int maxLen = n-1;
        // AC
        vector<int> g = {0};
        for(int i = 1;i<=n;i++) {
            int j0 = max(0, i-maxLen);
            for(auto& j: g) {
                if(j < j0) {
                    continue;
                }
                string target(word.begin()+j, word.begin()+i);
                //cout<<"search for "<<target<<endl;
                if(binary_search(words.begin(), words.end(), target)) {
                    g.push_back(i);
                    break;
                }
            }
        }
        return *g.rbegin() == n;
        // TLE
        vector<bool> f(n+1, false);
        f[0] = true;
        for(int i = 1;i<=n;i++) {
            int j0 = i-minLen;
            int jn = max(0, i-maxLen);
            for(int j = j0;j>=jn;j--) {
                if(!f[j]) {
                    continue;
                }
                string target(word.begin()+j, word.begin()+i);
                //cout<<"search for "<<target<<endl;
                if(!binary_search(words.begin(), words.end(), target))
                {
                    continue;
                }
                f[i] = true;
                break;
            }
        }
        return f[n];
    }
    vector<string> findAllConcatenatedWordsInADict(vector<string>& words) {
        vector<string> ret;
        sort(words.begin(), words.end());
        for(const auto& word: words) {
            if(isConcat(word, words)) {
                ret.emplace_back(word);
            }
        }
        return ret;
    }
};