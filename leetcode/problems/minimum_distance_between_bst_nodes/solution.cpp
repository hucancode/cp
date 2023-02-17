/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int minDiffInBST(TreeNode* root) {
        int ret = 1e5;
        stack<TreeNode*> next;
        while(root) {
            next.push(root);
            root = root->left;
        }
        while(!next.empty()) {
            auto a = next.top();
            next.pop();
            if(a->right) {
                auto b = a->right;
                while(b) {
                    next.push(b);
                    b = b->left;
                }
            }
            if(next.empty()) {
                continue;
            }
            auto b = next.top();
            ret = min(ret, abs(a->val - b->val));
        }
        return ret;
    }
};