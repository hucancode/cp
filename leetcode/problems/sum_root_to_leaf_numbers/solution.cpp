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
    int sumNumbers(TreeNode* root) {
        stack<TreeNode*> st;
        int ret = 0;
        int k = 0;
        st.push(root);
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            if(!node) {
                k /= 10;
                continue;
            }
            k = k*10 + node->val;
            if(!node->left && !node->right) {
                ret += k;
                continue;
            }
            if(node->left) {
                st.push(nullptr);
                st.push(node->left);
            }
            if(node->right) {
                st.push(nullptr);
                st.push(node->right);
            }
        }
        return ret;
    }
};