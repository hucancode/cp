/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        map<Node*, Node*> mp;
        auto node = head;
        auto clone = [&](Node* node) {
            if(node && mp.find(node) == mp.end()) {
                mp[node] = new Node(node->val);
            }
            return mp[node];
        };
        while(node) {
            clone(node);
            mp[node]->next = clone(node->next);
            mp[node]->random = clone(node->random);
            node = node->next;
        }
        return mp[head];
    }
};