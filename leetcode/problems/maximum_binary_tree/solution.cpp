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
    TreeNode* constructMaximumBinaryTree(vector<int>& nums) {
        if(nums.empty()) {
            return nullptr;
        }
        auto it = max_element(nums.begin(), nums.end());
        vector<int> a(nums.begin(), it);
        auto left = constructMaximumBinaryTree(a);
        vector<int> b(it+1, nums.end());
        auto right = constructMaximumBinaryTree(b);
        return new TreeNode(*it, left, right);
    }
};