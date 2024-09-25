class Node {
    Node* children[27];
public:
    int count;
    Node() {
        count = 0;
        memset(children, 0, sizeof(children));
    }
    Node* insert(char c) {
        auto k = c - 'a';
        if(!children[k]) {
            children[k] = new Node();
        }
        children[k]->count++;
        return children[k];
    }
    Node* query(char c) {
        auto k = c - 'a';
        return children[k];
    }
};

class Solution {
public:
    vector<int> sumPrefixScores(vector<string>& words) {
        auto trie = new Node();
        for(auto w: words) {
            auto node = trie;
            for(auto c: w) {
                node = node->insert(c);
            }
        }
        vector<int> ret;
        for(auto w: words) {
            int count = 0;
            auto node = trie;
            for(auto c: w) {
                node = node->query(c);
                if(node && node->count) {
                    count += node->count;
                }
            }
            ret.push_back(count);
        }
        return ret;
    }
};
