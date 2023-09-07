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
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        ListNode* low = nullptr;
        ListNode* high = nullptr;
        auto node = head;
        int i = 1;
        vector<ListNode*> v;
        while(node && i <= right) {
            if(i >= left && i<= right) {
                v.push_back(node);
                high = node->next;
            } else {
                low = node;
            }
            i++;
            node = node->next;
        }
        reverse(v.begin(), v.end());
        for(int i = 1;i<v.size();i++) {
            v[i-1]->next = v[i];
        }
        v.back()->next = high;
        if(low) {
            low->next = v[0];
            return head;
        }
        return v[0];
    }
};