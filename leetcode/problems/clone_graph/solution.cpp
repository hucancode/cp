/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/

class Solution {
public:
    Node* cloneGraph(Node* node) {
        if(!node) {
            return nullptr;
        }
        queue<Node*> q;
        map<Node*, Node*> copy;
        copy[node] = new Node(node->val);
        q.emplace(node);
        while(!q.empty()) {
            auto root = q.front();
            q.pop();
            for (auto child: root->neighbors) {
                if(!copy[child]) {
                    copy[child] = new Node(child->val);
                    q.emplace(child);
                }
                copy[root]->neighbors.push_back(copy[child]);
            }
        }
        return copy[node];
    }
};