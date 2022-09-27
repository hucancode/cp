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
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        vector<vector<int>> ret;
        if(!root) {
            return ret;
        }
        stack<vector<int>> paths;
        stack<TreeNode*> st;
        stack<int> remains;
        st.push(root);
        paths.push({root->val});
        remains.push(targetSum - root->val);
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            auto remain = remains.top();
            remains.pop();
            auto path = paths.top();
            paths.pop();
            if(!node->left && !node->right && remain == 0) {
                ret.push_back(path);
            }
            if(node->left) {
                st.push(node->left);
                remains.push(remain - node->left->val);
                auto next = path;
                next.push_back(node->left->val);
                paths.push(next);
            }
            if(node->right) {
                st.push(node->right);
                remains.push(remain - node->right->val);
                auto next = path;
                next.push_back(node->right->val);
                paths.push(next);
            }
        }
        return ret;
    }
};