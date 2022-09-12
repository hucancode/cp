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
    ListNode* reverse(ListNode* head) {
        if(!head) {
            return head;
        }
        auto a = head;
        auto b = head->next;
        while(b) {
            a->next = b->next;
            b->next = head;
            head = b;
            b = a->next;
        }
        return head;
    }
    bool isPalindrome(ListNode* head) {
        if(!head->next) {
            return true;
        }
        // find mid node
        auto mid = head;
        auto tail = head;
        while(tail) {
            tail = tail->next;
            if(tail) {
                tail = tail->next;
            }
            mid = mid->next;
        }
        mid = reverse(mid);
        while(mid) {
            if(mid->val != head->val) {
                return false;
            }
            mid = mid->next;
            head = head->next;
        }
        return true;
    }
};