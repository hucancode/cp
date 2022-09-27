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
    int calculate(TreeNode* root, long runningSum) {
        if(!root) {
            return 0;
        }
        int ret = 0;
        runningSum -= root->val;
        if(runningSum == 0) {
            ret = 1;
        }
        ret += calculate(root->left, runningSum);
        ret += calculate(root->right, runningSum);
        //cout<<"there are "<<ret<<" ways to make "<<runningSum<<" starting with "<<root->val<<endl;
        return ret;
    }
    int pathSum(TreeNode* root, int targetSum) {
        int ret = calculate(root, targetSum);
        if(root) {
            ret += pathSum(root->left, targetSum);
            ret += pathSum(root->right, targetSum);
        }
        return ret;
    }
};