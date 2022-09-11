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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* a = nullptr;
        ListNode* b = head;
        int distance = 0;
        while(b) {
            b = b->next;
            if(distance >= n) {
                if(a) {
                    a = a->next;
                } else {
                    a = head;
                }
            } else {
                distance++;
            }
        }
        if(a) {
            a->next = a->next->next;
            return head;
        }
        return head->next;
    }
};