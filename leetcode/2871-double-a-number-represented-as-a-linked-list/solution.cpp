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
    ListNode* doubleIt(ListNode* head) {
        ListNode* node = head;
        ListNode* prev = nullptr; 
        while(node) {
            int x = node->val*2;
            int r = x/10;
            if(r) {
                if(!prev) {
                    head = new ListNode(r, head);
                } else {
                    prev->val += r;
                }
            }
            node->val = x%10;
            prev = node;
            node = node->next;
        }
        return head;
    }
};
