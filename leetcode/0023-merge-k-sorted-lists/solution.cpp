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
        ListNode* root = nullptr;
        ListNode* node = nullptr;
        int n = lists.size();
        if(n == 0) {
            return root;
        }
        while(true) {
            int best = 0;
            for(int i = 0;i<n;i++) {
                auto bv = lists[best]?lists[best]->val:INT_MAX;
                auto iv = lists[i]?lists[i]->val:INT_MAX;
                if(iv < bv) {
                    best = i;
                }
            }
            if(!lists[best]) {
                break;
            }
            if(!root) {
                root = node = lists[best];
            } else {
                node = node->next = lists[best];
            }
            lists[best] = lists[best]->next;
        }
        return root;
    }
};
