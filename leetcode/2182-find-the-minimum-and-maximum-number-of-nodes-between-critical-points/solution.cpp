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
    vector<int> nodesBetweenCriticalPoints(ListNode* head) {
        int INF = 100000;
        int a = INF;
        int b = -1;
        int last = -1;
        int i = 0;
        int j = INF;
        ListNode *u = head, *v = nullptr, *w = nullptr;
        while(u) {
            i++;
            auto v = u->next;
            if(v) {
                w = v->next;
            }
            if(!v || !w) {
                break;
            }
            auto maxima = v->val > u->val && v->val > w->val;
            auto minima = v->val < u->val && v->val < w->val;
            u = v;
            if(!maxima && !minima) {
                continue;
            }
            if(last != -1) {
                a = min(a,  i - last);
                b = max(b, i - j);
            }
            j = min(j, i);
            last = i;
        }
        if(a == INF) {
            a = -1;
        }
        return {a, b};
    }
};
