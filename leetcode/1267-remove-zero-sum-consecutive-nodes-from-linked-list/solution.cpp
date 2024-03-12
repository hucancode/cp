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
    ListNode* removeZeroSumSublists(ListNode* head) {
        vector<ListNode*> vis;
        vector<int> sums {0};
        while(head) {
            auto sum = sums.back() + head->val;
            vis.push_back(head);
            sums.push_back(sum);
            auto n = distance(sums.begin(), find(sums.begin(), sums.end(), sum));
            vis.resize(n);
            sums.resize(n+1);
            head = head->next;
        }
        vis.push_back(nullptr);
        for(int i = 0;i<vis.size()-1;i++) {
            vis[i]->next = vis[i+1];
        }
        return vis.front();
    }
};
