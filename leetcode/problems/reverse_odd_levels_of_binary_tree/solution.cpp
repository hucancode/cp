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
    TreeNode* reverseOddLevels(TreeNode* root) {
        vector<TreeNode*> q;
        q.push_back(root);
        bool odd = false;
        while(!q.empty()) {
            int n = q.size();
            if(odd) {
                for(int i = 0;i<n/2;i++) {
                    swap(q[i]->val, q[n-i-1]->val);
                }
            }
            odd = !odd;
            for(int i = 0;i<n;i++) {
                if(!q[i]->left && !q[i]->right) {
                    continue;
                }
                q.push_back(q[i]->left);
                q.push_back(q[i]->right);
            }
            q.erase(q.begin(), q.begin()+n);
        }
        return root;
    }
};