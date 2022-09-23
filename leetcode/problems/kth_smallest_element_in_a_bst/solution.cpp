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
    int kthSmallest(TreeNode* root, int k) {
        stack<TreeNode*> st;
        auto node = root;
        // go to the furthest left (smallest item)
        while(node) {
            st.push(node);
            node = node->left;
        }
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            //cout<<"k = "<<k<<" node = "<<node->val<<endl;
            if(k-- == 1) {
                return node->val;
            }
            node = node->right;
            // go to the furthest left that is bigger than current node
            // aka the next smallest item
            while(node) {
                st.push(node);
                node = node->left;
            }
        }
        return 0;
    }
};