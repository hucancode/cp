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
    ListNode* split(ListNode* head, ListNode* a, ListNode* b) {
        ListNode* ret = nullptr;
        if(head->next) {
            ret = head->next->next;
        }
        a->next = head;
        b->next = head->next;
        if(a->next) {
            a->next->next = nullptr;
        }
        if(b->next) {
            b->next->next = nullptr;
        }
        return ret;
    }
    ListNode* oddEvenList(ListNode* head) {
        if(!head) {
            return nullptr;
        }
        if(!head->next) {
            return head;
        }
        auto oddHead = head;
        auto evenHead = head->next;
        auto odd = oddHead;
        auto even = evenHead;
        head = head->next->next;
        oddHead->next = nullptr;
        evenHead->next = nullptr;
        while(head) {
            head = split(head, odd, even);
            odd = odd->next;
            even = even->next;
        }
        odd->next = evenHead;
        return oddHead;
    }
};