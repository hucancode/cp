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
    bool findTarget(TreeNode* root, int k) {
        vector<int> f;
        f.reserve(10000);
        stack<TreeNode*> st;
        while(root) {
            st.push(root);
            root = root->left;
        }
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            auto key = k - node->val;
            if(binary_search(f.begin(), f.end(), key)) {
                return true;
            }
            f.push_back(node->val);
            node = node->right;
            while(node) {
                st.push(node);
                node = node->left;
            }
        }
        return false;
    }
};