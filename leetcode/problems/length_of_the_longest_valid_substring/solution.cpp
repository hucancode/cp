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
    int search(string& word, int i, int n) {
        int j = i;
        auto node = root;
        for(;j<n;j++) {
            if(node->hasChild(word[j])) {
                node = node->children[word[j]];
                if(node->ended) {
                    break;
                }
            } else {
                j = n;
                break;
            }
        }
        return j;
    }
};

class Solution {
public:
    int longestValidSubstring(string word, vector<string>& forbidden) {
        int n = word.size();
        Trie tree;
        for(auto x: forbidden) {
            tree.insert(x);
        }
        auto begin = word.begin() + n-1;
        auto end = word.end() + n-1;
        int ret = 0;
        for(int i = n-1;i>=0;i--) {
            int j = tree.search(word, i, n);
            //cout<<"check "<<i<<", n="<<n<<", "<<word.substr(i,n-i)<<" matched at "<<j<<endl;
            ret = max(ret, j-i);
            n = j;
        }
        return ret;
    }
};