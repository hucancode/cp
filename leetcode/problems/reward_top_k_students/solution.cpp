class Node {
public:
    bool ended;
    map<char, Node*> children;
    bool hasChild(char c) {
        return children.find(c) != children.end();
    }
    Node* makeChild(char c) {
        auto ret = new Node();
        ret->ended = false;
        children[c] = ret;
        return ret;
    }
};

class Trie {
public:
    Node* root;
    Trie() {
        root = new Node();
    }
    
    void insert(string& word) {
        auto node = root;
        for(int i = 0;i<word.size();i++) {
            if(node->hasChild(word[i])) {
                node = node->children[word[i]];
            } else {
                node = node->makeChild(word[i]);
            }
        }
        node->ended = true;
    }
    
    bool search(string& word) {
        auto node = root;
        for(int i = 0;i<word.size();i++) {
            if(node->hasChild(word[i])) {
                node = node->children[word[i]];
            } else {
                return false;
            }
        }
        return node->ended;
    }
    
    bool startsWith(string& prefix) {
        auto node = root;
        for(int i = 0;i<prefix.size();i++) {
            if(node->hasChild(prefix[i])) {
                node = node->children[prefix[i]];
            } else {
                return false;
            }
        }
        return true;
    }
};
class Solution {
public:
    vector<int> topStudents(vector<string>& good, vector<string>& bad, vector<string>& report, vector<int>& ids, int k) {
        Trie goodTree;
        Trie badTree;
        for(auto& s: good) {
            goodTree.insert(s);
        }
        for(auto& s: bad) {
            badTree.insert(s);
        }
        int n = ids.size();
        vector<int> ret(k);
        vector<pair<int,int>> scores(n);
        for(int i = 0;i<n;i++) {
            scores[i].second = ids[i];
            scores[i].first = 0;
            auto& s = report[i];
            int l = 0;
            while(l < s.size()) {
                int r = s.find(' ',l);
                auto token = s.substr(l, r-l);
                if(goodTree.search(token)) {
                    scores[i].first+= 3;
                } else if(badTree.search(token)) {
                    scores[i].first-=1;
                }
                if(r == string::npos) {
                    break;
                }
                l = r+1;
            }
        }
        sort(scores.begin(), scores.end(), [](const pair<int,int> a, const pair<int,int> b) -> bool {
            if(a.first > b.first) {
                return true;
            }
            if(a.first < b.first) {
                return false;
            }
            return a.second < b.second;
        });
        for(int i = 0;i<k;i++) {
            ret[i] = scores[i].second;
        }
        return ret;
    }
};