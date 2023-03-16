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
typedef vector<int>::iterator vii;
class Solution {
public:
    void traverse(TreeNode*& root, vii a, vii b, vii c, vii d) {
        int n = distance(a, b);
        if(n == 0) {
            return;
        }
        root = new TreeNode(*(d-1));
        int i = distance(a, find(a, b, root->val));
        traverse(root->left, 
            a, a+i,
            c, c+i);
        traverse(root->right, 
            a+i+1, b,
            c+i, d-1);
    }
    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        TreeNode* root = nullptr;
        traverse(root, inorder.begin(), inorder.end(), postorder.begin(), postorder.end());
        return root;
    }
};