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
    bool isValidBST(TreeNode* root) {
        stack<long> localMaxes;
        stack<long> localMins;
        stack<TreeNode*> st;
        st.push(root);
        localMaxes.push(INT_MAX+1L);
        localMins.push(INT_MIN-1L);
        while(!st.empty()) {
            auto localMin = localMins.top();
            auto localMax = localMaxes.top();
            auto node = st.top();
            //cout<<"travel node "<<node->val<<", local min = "<<localMin<<", local max = "<<localMax<<endl;
            st.pop();
            localMins.pop();
            localMaxes.pop();
            if(node->left) {
                auto v = node->left->val;
                if(v >= node->val || v <= localMin || v >= localMax) {
                    //cout<<"left child "<<v<<" failed"<<endl;
                    return false;
                }
                st.push(node->left);
                localMins.push(localMin);
                localMaxes.push(min(localMax, (long)node->val));
            }
            if(node->right) {
                auto v = node->right->val;
                if(v <= node->val || v <= localMin || v >= localMax) {
                    //cout<<"right child "<<v<<" failed"<<endl;
                    return false;
                }
                st.push(node->right);
                localMins.push(max(localMin, (long)node->val));
                localMaxes.push(localMax);
            }
            
        }
        return true;
    }
};