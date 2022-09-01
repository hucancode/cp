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
    ListNode *detectCycle(ListNode *head) {
        if(!head) {
            return nullptr;
        }
        stack<ListNode*> sa;
        stack<ListNode*> sb;
        sa.push(head);
        sb.push(head->next);
        bool jump = true;
        while(sb.top()) {
            if(sa.top() == sb.top()) {
                auto ret = sa.top();
                while(!sa.empty() && sa.top() == sb.top()) {
                    ret = sa.top();
                    sa.pop();
                    sb.pop();
                }
                return ret;
            }
            sb.push(sb.top()->next);
            if(jump) {
                sa.push(sa.top()->next);
            }
            jump = !jump;
        }
        return nullptr;
    }
};