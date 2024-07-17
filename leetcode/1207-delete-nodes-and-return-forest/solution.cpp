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
    vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        vector<TreeNode*> ret;
        set<int> black_list(to_delete.begin(), to_delete.end());
        stack<tuple<TreeNode*, TreeNode*, bool>> q;
        q.push(make_tuple(nullptr, root, false));
        TreeNode* parent;
        TreeNode* node;
        bool is_left;
        while(!q.empty()) {
            tie(parent, node, is_left) = q.top();
            q.pop();
            if(!node) {
                continue;
            }
            if(black_list.contains(node->val)) {
                if(parent) {
                    if(is_left) {
                        parent->left = nullptr;
                    } else {
                        parent->right = nullptr;
                    }
                }
                q.push(make_tuple(nullptr, node->left, true));
                q.push(make_tuple(nullptr, node->right, false));
            } else {
                if(!parent) {
                    ret.push_back(node);
                }
                q.push(make_tuple(node, node->left, true));
                q.push(make_tuple(node, node->right, false));
            }
        }
        return ret;
    }
};
