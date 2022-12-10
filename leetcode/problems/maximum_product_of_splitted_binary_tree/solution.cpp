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
    map<TreeNode*, int> sum;
public:
    int traverse(TreeNode* root) {
        if(!root) {
            return 0;
        }
        sum[root] = root->val + traverse(root->left) + traverse(root->right);
        return sum[root];
    }
    int maxProduct(TreeNode* root) {
        const int INF = 1e9+7;
        long long ret = 0;
        traverse(root);
        queue<TreeNode*> q;
        q.push(root);
        while(!q.empty()) {
            auto node = q.front();
            q.pop();
            if(!node) {
                continue;
            }
            if(node->left) {
                long long a = sum[node->left];
                long long b = sum[root] - a;
                ret = max(ret,a*b);
                q.push(node->left);
            }
            if(node->right) {
                long long a = sum[node->right];
                long long b = sum[root] - a;
                ret = max(ret,a*b);
                q.push(node->right);
            }
        }
        return ret%INF;
    }
};