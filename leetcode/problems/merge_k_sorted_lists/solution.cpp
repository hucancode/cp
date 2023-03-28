/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        auto sorter = [](ListNode* a, ListNode* b) {
            return a->val > b->val;
        };
        priority_queue<
            ListNode*, 
            vector<ListNode*>, 
            decltype(sorter)>
            q(sorter);
        for(auto node: lists) {
            if(node) q.push(node);
        }
        ListNode *root = nullptr, *current = nullptr;
        while(!q.empty()) {
            auto node = q.top();
            q.pop();
            if(current) current = current->next = node;
            else root = current = node;
            if(node->next) q.push(node->next);
        }
        return root;
    }
};