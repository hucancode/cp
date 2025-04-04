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
    TreeNode* lcaDeepestLeaves(TreeNode* root) {
        map<int, TreeNode*> tree;
        tree[1] = root;
        vector<int> q = {1};
        while(true) {
            vector<int> next;
            for(int j: q) {
                if(tree[j]->left) {
                    tree[j*2] = tree[j]->left;
                    next.push_back(j*2);
                }
                if(tree[j]->right) {
                    tree[j*2+1] = tree[j]->right;
                    next.push_back(j*2+1);
                }
            }
            if(next.empty()) {
                break;
            }
            q = next;
        }
        while(q.size() > 1) {
            vector<int> next;
            for(int i: q) {
                if(!next.empty() && next[next.size() - 1] == i/2) {
                    continue;
                }
                next.push_back(i/2);
                q = next;
            }
        }
        return tree[q[0]];
    }
};
