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
    string tree2str(TreeNode* root) {
        ostringstream ret;
        stack<TreeNode*> st;
        st.push(root);
        bool first_blood = true;
        while(!st.empty()) {
            root = st.top();
            st.pop();
            if(!first_blood) {
                ret << (root?'(':')');
                if(root) st.push(nullptr);
            }
            first_blood = false;
            if(!root) {
                continue;
            }
            ret<<root->val;
            if(!root->left && !root->right) {
                continue;
            }
            if(root->right) {
                st.push(root->right);
            }
            if(root->left) {
                st.push(root->left);
            } else {
                ret << "()";
            }
        }
        return ret.str();
    }
};