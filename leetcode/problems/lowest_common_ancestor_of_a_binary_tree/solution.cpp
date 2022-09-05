/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        map<TreeNode*, TreeNode*> parent;
        queue<TreeNode*> visit;
        visit.push(root);
        while(!visit.empty()) {
            auto node = visit.front();
            if(node->left) {
                parent[node->left] = node;
                visit.push(node->left);
            }
            if(node->right) {
                parent[node->right] = node;
                visit.push(node->right);
            }
            visit.pop();
        }
        set<TreeNode*> path;
        path.insert(p);
        while(p != root) {
            p = parent[p];
            path.insert(p);
        }
        while(q != root) {
            if(path.find(q) != path.end()) {
                break;
            }
            q = parent[q];
        }
        return q;
    }
};