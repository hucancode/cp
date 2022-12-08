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
    vector<int> getLeaf(TreeNode* root) {
        vector<int> ret;
        stack<TreeNode*> q;
        q.push(root);
        while(!q.empty()) {
            auto node = q.top();
            q.pop();
            if(!node) {
                continue;
            }
            if(!node->left && !node->right) {
                ret.push_back(node->val);
                continue;
            }
            q.push(node->left);
            q.push(node->right);
        }
        return ret;
    }
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        auto a = getLeaf(root1);
        auto b = getLeaf(root2);
        if(a.size() != b.size()) {
            return false;
        }
        auto i = a.begin();
        auto j = b.begin();
        while(i != a.end() && j != b.end()) {
            if(*i != *j) {
                return false;
            }
            i++;
            j++;
        }
        return true;
    }
};