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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int a, b, c;
        a = l1->val;
        b = l2->val;
        c = a+b;
        int carry = c/10;
        c = c%10;
        ListNode* head = new ListNode(c);
        auto node = head;
        l1 = l1->next;
        l2 = l2->next;
        while(l1 || l2) {
            a = l1?l1->val:0;
            b = l2?l2->val:0;
            c = a+b+carry;
            carry = c/10;
            c = c%10;
            node->next = new ListNode(c);
            node = node->next;
            if(l1) {
                l1 = l1->next;
            }
            if(l2) {
                l2 = l2->next;
            }
        }
        if(carry != 0) {
            node->next = new ListNode(carry);
        }
        return head;
    }
};