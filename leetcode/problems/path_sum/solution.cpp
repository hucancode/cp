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
    bool hasPathSum(TreeNode* root, int targetSum) {
        if(!root) {
            return false;
        }
        stack<TreeNode*> st;
        stack<int> remains;
        st.push(root);
        remains.push(targetSum - root->val);
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            auto remain = remains.top();
            remains.pop();
            if(!node->left && !node->right && remain == 0) {
                return true;
            }
            if(node->left) {
                st.push(node->left);
                remains.push(remain - node->left->val);
            }
            if(node->right) {
                st.push(node->right);
                remains.push(remain - node->right->val);
            }
        }
        return false;
    }
};