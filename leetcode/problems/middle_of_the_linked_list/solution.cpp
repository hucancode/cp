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
    ListNode* middleNode(ListNode* head)
    {
        ListNode* mid = head;
        while(head)
        {
            head = head->next;
            if(head)
            {
                head = head->next;
                mid = mid->next;
            }
        }
        return mid;
    }
};