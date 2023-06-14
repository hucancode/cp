class Node {
public:
    char value;
    int count;
    map<char, Node*> children;
    Node(char v=0) {
        value = v;
        count = 0;
    }
    Node* get(char v) {
        return children[v];
    }
    Node* insert(char v) {
        if(!children[v]) children[v] = new Node(v);
        children[v]->count++;
        return children[v];
    }
};
class Solution {
public:
    vector<int> sumPrefixScores(vector<string>& words) {
        auto trie = new Node();
        for(auto word: words) {
            auto node = trie;
            for(char c: word) {
                node = node->insert(c);
            }
        }
        vector<int> ret;
        ret.reserve(words.size());
        for(auto word: words) {
            auto node = trie;
            auto score = 0;
            for(char c: word) {
                node = node->get(c);
                score += node->count;
            }
            ret.push_back(score);
        }
        return ret;
    }
};