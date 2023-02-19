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
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        vector<vector<int>> ret;
        vector<vector<TreeNode*>> q;
        q.push_back({root});
        while(!q.back().empty()) {
            auto vis = q.back();
            ret.push_back({});
            q.push_back({});
            for(auto node: vis) {
                if(!node) continue;
                ret.back().push_back(node->val);
                q.back().push_back(node->left);
                q.back().push_back(node->right);
            }
        }
        if(ret.back().empty()) {
            ret.pop_back();
        }
        for(int i = 1;i<ret.size();i+=2) {
            reverse(ret[i].begin(), ret[i].end());
        }
        return ret;
    }
};