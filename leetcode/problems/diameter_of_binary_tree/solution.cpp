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
    int longestPath(TreeNode* root) {
        if(!root) {
            return -1;
        }
        return max(longestPath(root->left), longestPath(root->right)) + 1;
    }
    int diameterOfBinaryTree(TreeNode* root) {
        if(!root) {
            return 0;
        }
        int len = longestPath(root->left) + longestPath(root->right) + 2;
        int diameterLeft = diameterOfBinaryTree(root->left);
        int diameterRight = diameterOfBinaryTree(root->right);
        return max(len, max(diameterLeft, diameterRight));
    }
};