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
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        if(!list1) {
            return list2;
        }
        if(!list2) {
            return list1;
        }
        if(list1->val > list2->val) {
            swap(list1, list2);
        }
        ListNode* root = list1;
        while(true) {
            if(!list2) {
                break;
            }
            if(!list1->next) {
                list1->next = list2;
                break;
            }
            if(list1->next->val > list2->val) {
                swap(list1->next, list2);
            }
            list1 = list1->next;
        }
        return root;
    }
};