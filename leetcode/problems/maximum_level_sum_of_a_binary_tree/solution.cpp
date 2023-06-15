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
    int maxLevelSum(TreeNode* root) {
        if(!root) {
            return 0;
        }
        queue<TreeNode*> q;
        q.emplace(root);
        int ret = 1;
        int best = root->val;
        int level = 1;
        while(!q.empty()) {
            int n = q.size();
            int score = 0;
            while(n--) {
                auto node = q.front();
                q.pop();
                score += node->val;
                if(node->left) q.emplace(node->left);
                if(node->right) q.emplace(node->right);
            }
            if(score > best) {
                ret = level;
                best = score;
            }
            level += 1;
        }
        return ret;
    }
};