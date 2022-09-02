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
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> ret;
        if(!root) {
            return ret;
        }
        queue<vector<TreeNode*>> q;
        q.push({root});
        while(!q.empty()) {
            auto nodes = q.front();
            vector<int> values;
            vector<TreeNode*> children;
            q.pop();
            for(int i = 0;i<nodes.size();i++) {
                values.push_back(nodes[i]->val);
                if(nodes[i]->left) {
                    children.push_back(nodes[i]->left);
                }
                if(nodes[i]->right) {
                    children.push_back(nodes[i]->right);
                }
            }
            ret.push_back(values);
            if(children.size() > 0) {
                q.push(children);
            }
        }
        return ret;
    }
};