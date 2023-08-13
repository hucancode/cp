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
        stack<ListNode*> st;
        while(head) {
            st.push(head);
            head = head->next;
        }
        int remainder = 0;
        while(!st.empty()) {
            head = st.top();
            st.pop();
            int x = head->val;
            int y = x*2+remainder;
            remainder = y/10;
            head->val = y%10;
        }
        if(remainder != 0) {
            head = new ListNode(remainder, head);
        }
        return head;
    }
};