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
    ListNode* reverseList(ListNode* head, int k) {
        if(!head) {
            return head;
        }
        auto a = head;
        auto b = head->next;
        int i = 1;
        while(b && i < k) {
            a->next = b->next;
            b->next = head;
            head = b;
            b = a->next;
            i++;
        }
        return head;
    }
    void print(ListNode* head) {
        cout<<"list = ";
        while(head) {
            cout<<head->val<<" ";
            head = head->next;
        }
        cout<<endl;
    }
    ListNode* reverseKGroup(ListNode* head, int k) {
        ListNode* firstHead = head;
        ListNode* lastSegmentTail = nullptr;
        while(head) {
            //print(firstHead);
            int i = 1;
            auto node = head;
            while(i++ < k) {
                if(!node) {
                    break;
                }
                node = node->next;
            }
            if(!node) {
                break;
            }
            //cout<<"reverse "<<head->val<<" - "<<node->val<<endl;
            head = reverseList(head, k);
            if(lastSegmentTail != nullptr) {
                lastSegmentTail->next = head;
            } else {
                firstHead = head;
            }
            i = 1;
            while(i<=k) {
                lastSegmentTail = head;
                head = head->next;
                i++;
            }
            
        }
        return firstHead;
    }
};