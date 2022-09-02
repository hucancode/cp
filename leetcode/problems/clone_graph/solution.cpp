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
        const int n = 101;
        Node* nodes[n];
        Node* clones[n];
        fill(nodes, nodes+n, nullptr);
        queue<Node*> q;
        q.push(node);
        while(!q.empty()) {
            int v = q.front()->val;
            if(!nodes[v]) {
                nodes[v] = q.front();
                clones[v] = new Node(v);
                for(int i = 0;i<q.front()->neighbors.size();i++) {
                    q.push(q.front()->neighbors[i]);
                }
            }
            q.pop();
        }
        for(int i = 0;i<n;i++) {
            if(!nodes[i]) {
                continue;
            }
            for(int j = 0;j<nodes[i]->neighbors.size();j++) {
                auto v = nodes[i]->neighbors[j]->val;
                clones[i]->neighbors.push_back(clones[v]);
            }
        }
        return clones[1];
    }
};