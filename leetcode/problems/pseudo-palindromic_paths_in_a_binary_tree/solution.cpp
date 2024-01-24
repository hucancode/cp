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
    int pseudoPalindromicPaths (TreeNode* root) {
        stack<TreeNode*> q;
        vector<int> path;
        vector<int> count(10, 0);
        int ret = 0;
        q.push(root);
        while(!q.empty()) {
            auto node = q.top();
            q.pop();
            if(!node) {
                auto x = path.back();
                path.pop_back();
                count[x]--;
                continue;
            }
            q.push(nullptr);
            path.push_back(node->val);
            count[node->val]++;
            if(node->left) q.push(node->left);
            if(node->right) q.push(node->right);
            if(!node->left && !node->right && 
                count_if(count.begin(), count.end(), [](auto x) { return x%2; }) <= 1) {
                ret++;
            }
        }
        return ret;
    }
};