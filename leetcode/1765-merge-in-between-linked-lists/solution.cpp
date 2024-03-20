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
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        auto node_a = list1;
        int i = 1;
        while(i++ < a) node_a = node_a->next;
        auto node_b = node_a->next;
        while(i++ <= b+1) node_b = node_b->next;
        node_a->next = list2;
        while(node_a->next) node_a = node_a->next;
        node_a->next = node_b;
        return list1;
    }
};
