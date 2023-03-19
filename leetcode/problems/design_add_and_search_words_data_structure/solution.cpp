class Node {
    vector<Node*> children;
    bool isLeaf;
public:
    Node() {
        isLeaf = false;
    }
    void push(string::iterator i, string::iterator end) {
        if(i == end) {
            isLeaf = true;
            return;
        }
        auto c = *i - 'a';
        auto j = i+1;
        if(children.size() <= c) {
            children.resize(c+1, nullptr);
        }
        if(!children[c]) {
            children[c] = new Node();
        }
        children[c]->push(j, end);
    }
    bool search(string::iterator i, string::iterator end) {
        if(i == end) {
            return isLeaf;
        }
        auto c = *i;
        auto j = i+1;
        if(c != '.') {
            c -= 'a';
            return children.size() > c && children[c] && children[c]->search(j, end);
        }
        for(auto child: children) {
            if(child && child->search(j, end)) {
                return true;
            }
        }
        return false;
    }
};

class WordDictionary {
    Node root;
public:
    WordDictionary() {
    }
    
    void addWord(string word) {
        root.push(word.begin(), word.end());
    }
    
    bool search(string word) {
        return root.search(word.begin(), word.end());
    }
};

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary* obj = new WordDictionary();
 * obj->addWord(word);
 * bool param_2 = obj->search(word);
 */