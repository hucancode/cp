/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    bool hasCycle(ListNode *head) {
        if(!head) {
            return false;
        }
        auto a = head;
        auto b = a->next;
        bool jump = false;
        while(b) {
            if(a == b) {
                return true;
            }
            b = b->next;
            if(jump = !jump) {
                a = a->next;
            }
        }
        return false;
    }
};