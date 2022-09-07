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
    int maxPathSum(TreeNode* root) {
        map<TreeNode*, int> take2;
        map<TreeNode*, int> take1;
        stack<TreeNode*> st;
        st.push(root);
        stack<TreeNode*> vis;
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            vis.push(node);
            if(node->left) {
                st.push(node->left);
            }
            if(node->right) {
                st.push(node->right);
            }
        }
        while(!vis.empty()) {
            auto node = vis.top();
            vis.pop();
            if(!node->left && !node->right) {
                take1[node] = node->val;
                take2[node] = node->val;
            }
            int takeNone = node->val;
            int takeLeft = -1;
            int takeRight = -1;
            if(node->left) {
                takeLeft = take1[node->left];
            }
            if(node->right) {
                takeRight = take1[node->right];
            }
            int takeBoth = takeLeft + takeRight;
            take1[node] = takeNone + max(0, max(takeLeft, takeRight));
            take2[node] = takeNone + max(0, max(takeBoth, max(takeLeft, takeRight)));
        }
        int ret = take2[root];
        for(const auto& x: take2) {
            ret = max(ret, x.second);
        }
        return ret;
    }
};