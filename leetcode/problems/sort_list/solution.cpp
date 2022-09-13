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
    void print(ListNode* head) {
        auto node = head;
        while(node) {
            cout<<node->val<<" ";
            node = node->next;
        }
    }
    
    ListNode* merge(ListNode* left, ListNode* right) {
        if(!left) {
            return right;
        }
        if(!right) {
            return left;
        }
        ListNode* head;
        if(left->val < right->val) {
            head = left;
            left = left->next;
        } else {
            head = right;
            right = right->next;
        }
        auto node = head;
        while(true) {
            if(!left) {
                node->next = right;
                break;
            }
            if(!right) {
                node->next = left;
                break;
            }
            if(left->val < right->val) {
                node->next = left;
                left = left->next;
            } else {
                node->next = right;
                right = right->next;
            }
            node = node->next;
        }
        return head;
    }
    
    ListNode* split(ListNode* head) {
        auto a = head;
        auto b = head->next;
        while(b) {
            b = b->next;
            if(b) {
                b = b->next;
            } else {
                break;
            }
            a = a->next;
        }
        auto ret = a->next;
        a->next = nullptr;
        return ret;
    }
    
    ListNode* sortList(ListNode* head) {
        // cout<<"sorting: ";
        // print(head);
        // cout<<endl;
        if(!head) {
            return head;
        }
        if(!head->next) {
            return head;
        }
        auto left = head;
        auto right = split(head);
        // cout<<"left = ";
        // print(left);
        // cout<<endl;
        // cout<<"right = ";
        // print(right);
        // cout<<endl;
        left = sortList(left);
        right = sortList(right);
        return merge(left, right);
    }
};