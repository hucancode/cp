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
    vector<double> averageOfLevels(TreeNode* root) {
        queue<TreeNode*> q;
        vector<double> ret;
        q.push(root);
        while(!q.empty()) {
            int n = q.size();
            double sum = 0;
            for(int i = 0;i<n;i++) {
                auto top = q.front();
                sum += top->val;
                if(top->left) {
                    q.push(top->left);
                }
                if(top->right) {
                    q.push(top->right);
                }
                q.pop();
            }
            sum /= n;
            ret.push_back(sum);
        }
        return ret;
    }
};