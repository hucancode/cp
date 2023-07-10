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
    int minDepth(TreeNode* root) {
        int ret = 0;
        if(!root) {
            return ret;
        }
        queue<TreeNode*> q;
        q.push(root);
        ret++;
        while(!q.empty()) {
            int n = q.size();
            while(n--) {
                auto node = q.front();
                q.pop();
                if(!node) {
                    continue;
                }
                if(!node->left && !node->right) {
                    return ret;
                }
                q.push(node->left);
                q.push(node->right);
            }
            ret++;
        }
        return 0;
    }
};