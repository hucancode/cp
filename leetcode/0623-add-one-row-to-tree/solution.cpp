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
    TreeNode* addOneRow(TreeNode* root, int val, int depth) {
        if(depth <= 1) {
            return new TreeNode(val, root, nullptr);
        }
        deque<TreeNode*> q = { root };
        while(--depth > 1) {
            auto n = q.size();
            while(n--) {
                auto node = q.front();
                q.pop_front();
                if(!node) continue;
                q.push_back(node->left);
                q.push_back(node->right);
            }
        }
        for(auto node: q) {
            if(!node) continue;
            node->left = new TreeNode(val, node->left, nullptr);
            node->right = new TreeNode(val, nullptr, node->right);
        }
        return root;
    }
};
