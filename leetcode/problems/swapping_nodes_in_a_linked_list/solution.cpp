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
    ListNode* swapNodes(ListNode* head, int k) {
        auto turtle_a = head;
        while(--k) {
            turtle_a = turtle_a->next;
        }
        auto rabbit_b = turtle_a;
        auto turtle_b = head;
        while(rabbit_b->next) {
            rabbit_b = rabbit_b->next;
            turtle_b = turtle_b->next;
        }
        swap(turtle_a->val, turtle_b->val);
        return head;
    }
};