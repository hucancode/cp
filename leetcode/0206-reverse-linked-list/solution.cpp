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
    ListNode* reverseList(ListNode* head) {
        if(!head) {
            return head;
        }
        auto node = head->next;
        head->next = nullptr;
        while(node) {
            auto next = node->next;
            node->next = head;
            head = node;
            node = next;
        }
        return head;
    }
};
