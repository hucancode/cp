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
    vector<int> postorderTraversal(TreeNode* root) {
        vector<int> ret;
        stack<TreeNode*> st;
        st.push(root);
        while(!st.empty()) {
            auto x = st.top();
            st.pop();
            if(!x) {
                continue;
            }
            if(!x->left && !x->right) {
                ret.push_back(x->val);
            } else {
                st.push(x);
                st.push(x->right);
                st.push(x->left);
                x->left = nullptr;
                x->right = nullptr;
            }
        }
        return ret;
    }
};