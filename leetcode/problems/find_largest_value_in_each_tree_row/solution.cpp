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
    vector<int> largestValues(TreeNode* root) {
        vector<int> ret;
        if(!root) return ret;
        queue<TreeNode*> q;
        q.push(root);
        while(!q.empty()) {
            int n = q.size();
            int x = INT_MIN;
            while(n--) {
                auto node = q.front();
                q.pop();
                if(node) {
                    int y = node->val;
                    x = max(x, y);
                    if(node->left) q.push(node->left);
                    if(node->right) q.push(node->right);
                }
            }
            ret.push_back(x);
        }
        return ret;
    }
};