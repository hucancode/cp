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
    bool removeChild(char c) {
        if(!hasChild(c)) {
            return false;
        }
        if(!children[c]->children.empty()) {
            return false;
        }
        delete children[c];
        children.erase(children.find(c));
        return true;
    }
};

class Trie {
public:
    Node* root;
    Trie() {
        root = new Node();
    }
    
    bool empty() {
        return root->children.empty();
    }
    
    void remove(string word) {
        auto node = root;
        stack<Node*> st;
        st.push(root);
        for(int i = 0;i<word.size()-1;i++) {
            if(node->hasChild(word[i])) {
                node = node->children[word[i]];
                st.push(node);
            } else {
                return;
            }
        }
        for(int i = word.size() - 1;i>=0;i--) {
            node = st.top();
            st.pop();
            if(!node->removeChild(word[i])) {
                break;
            }
        }
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
    
    Node* search(string word) {
        auto node = root;
        for(int i = 0;i<word.size();i++) {
            if(node->hasChild(word[i])) {
                node = node->children[word[i]];
            } else {
                return nullptr;
            }
        }
        return node;
    }
};

class Solution {
public:
    bool exist(vector<vector<char>>& board, string target) {
        vector<bool> alphabet(256, false);
        set<string> ret;
        int m = board.size();
        int n = board[0].size();
        for(const auto& x: target) {
            alphabet[x] = true;
        }
        vector<vector<bool>> vis(m, vector<bool>(n, false));
        stack<int> st;
        for(int i = 0;i<m;i++) {
            for(int j = 0;j<n;j++) {
                int key = board[i][j];
                if(!alphabet[key]) {
                    continue;
                }
                st.emplace(i);
                st.emplace(j);
            }
        }
        string word;
        word.reserve(15);
        while(!st.empty()) {
            auto j = st.top();
            st.pop();
            auto i = st.top();
            st.pop();
            if(vis[i][j]) {
                vis[i][j] = false;
                word.pop_back();
                continue;
            }
            word.push_back(board[i][j]);
            vis[i][j] = true;
            bool shouldGo = target.rfind(word,0) != string::npos;
            //cout<<"word: "<<word<<", should go = "<<shouldGo<<endl;
            if(!shouldGo) {
                vis[i][j] = false;
                word.pop_back();
                continue;
            }
            st.emplace(i);
            st.emplace(j);
            if(i > 0 && !vis[i-1][j]) {
                st.emplace(i-1);
                st.emplace(j);
            }
            if(i < m-1 && !vis[i+1][j]) {
                st.emplace(i+1);
                st.emplace(j);
            }
            if(j > 0 && !vis[i][j-1]) {
                st.emplace(i);
                st.emplace(j-1);
            }
            if(j < n-1 && !vis[i][j+1]) {
                st.emplace(i);
                st.emplace(j+1);
            }
            if(word == target) {
                return true;
            }
        }
        return false;
    }
};