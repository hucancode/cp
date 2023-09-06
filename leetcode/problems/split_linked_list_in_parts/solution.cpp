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
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        int n = 0;
        auto node = head;
        while(node) node = node->next, n++;
        int remainder = n%k;
        int len = n/k;
        vector<ListNode*> ret;
        node = head;
        auto tail = node;
        while(k--) {
            ret.push_back(node);
            int n = len;
            if(remainder) remainder--, n++;
            while(n-- && node) tail = node, node = node->next;
            if(tail) tail->next = nullptr;
        }
        return ret;
    }
};