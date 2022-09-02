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
    
    void insert(string word) {
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
    
    bool search(string word) {
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
    
    bool startsWith(string prefix) {
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

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */