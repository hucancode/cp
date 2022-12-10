#define BIT_COUNT 32
class Node {
public:
    bool ended;
    map<bool, Node*> children;
    bool hasChild(bool c) {
        return children.find(c) != children.end();
    }
    Node* makeChild(bool c) {
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
    
    void insert(int word) {
        //cout<<"insert "<<bitset<BIT_COUNT>(word)<<endl;
        auto node = root;
        for(int i = 0;i<BIT_COUNT;i++) {
            int k = (1<<(BIT_COUNT-i-1));
            bool bit = word & k;
            //cout<<"k = "<<bitset<BIT_COUNT>(k)<<" bit "<<i<<" = "<<bit<<endl;
            if(node->hasChild(bit)) {
                node = node->children[bit];
            } else {
                node = node->makeChild(bit);
            }
        }
        node->ended = true;
    }
    
    bool search(int word) {
        auto node = root;
        int k = (1<<(BIT_COUNT-1));
        for(int i = 0;i<BIT_COUNT;i++) {
            bool bit = word & k;
            if(node->hasChild(bit)) {
                node = node->children[bit];
            } else {
                return false;
            }
            k = k>>1;
        }
        return node->ended;
    }
    
    int searchBestMatch(int word) {
        auto node = root;
        int ret = 0;
        for(int i = 0;i<BIT_COUNT;i++) {
            int j = BIT_COUNT-i-1;
            int k = (1<<j);
            bool bit = word & k;
            if(node->hasChild(bit)) {
                node = node->children[bit];
                k = bit<<j;
            } else {
                node = node->children[!bit];
                k = (!bit)<<j;
            }
            ret = ret | k;
        }
        return ret;
    }
};

class Solution {
public:
    int findMaximumXOR(vector<int>& nums) {
        int ret = 0;
        int n = nums.size();
        Trie tree;
        for(auto x: nums) {
            tree.insert(x);
        }
        for(auto x: nums) {
            auto y = tree.searchBestMatch(~x);
            //cout<<"check "<<x<<"-"<<bitset<BIT_COUNT>(x)<<" ideal match = "<<bitset<BIT_COUNT>(~x)<<" found "<<y<<"-"<<bitset<BIT_COUNT>(y)<<endl;
            ret = max(ret, x^y);
        }
        return ret;
    }
};