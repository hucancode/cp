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
    int maxAncestorDiff(TreeNode* root) {
        int ret = 0;
        stack<tuple<TreeNode*,int,int>> st;
        st.emplace(root, root->val, root->val);
        while(!st.empty()) {
            TreeNode* node;
            int a, b;
            tie(node,a,b) = st.top();
            st.pop();
            if(!node) {
                continue;
            }
            cout<<"check "<<node->val<<endl;
            a = min(a, node->val);
            b = max(b, node->val);
            ret = max(ret, abs(a-b));
            st.emplace(node->left, a, b);
            st.emplace(node->right, a, b);
        }
        return ret;
    }
};