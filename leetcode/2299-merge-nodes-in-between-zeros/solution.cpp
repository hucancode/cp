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
    ListNode* mergeNodes(ListNode* head) {
        auto ret = head;
        auto a = head->next;
        auto b = ret;
        while(a) {
            int x = a->val;
            if(x) {
                b->val += x;
            } else if(a->next) {
                b = b->next;
                b->val = 0;
            } else {
                b->next = nullptr;
            }
            a = a->next;
        }
        return ret;
    }
};
