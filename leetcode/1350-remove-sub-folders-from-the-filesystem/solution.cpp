class Trie {
    bool is_leaf;
    map<string, Trie> children;
public:
    Trie() {
        is_leaf = false;
    }
    bool insert(string::iterator begin, string::iterator end) {
        if(is_leaf) {
            return false;
        }
        auto i = find(begin, end, '/');
        if(i == end) {
            children[string(begin, end)].is_leaf = true;
            return true;
        } else {
            return children[string(begin, i)].insert(i+1, end);
        }
    }
};
class Solution {
public:
    vector<string> removeSubfolders(vector<string>& folder) {
        sort(folder.begin(), folder.end(), [](string& a, string& b) {
            return a.size() < b.size();
        });
        vector<string> ret;
        Trie root;
        for(auto s: folder) {
            if(root.insert(s.begin(), s.end())) {
                ret.push_back(s);
            }
        }
        return ret;
    }
};
