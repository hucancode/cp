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
    vector<int> distanceK(TreeNode* root, TreeNode* target, int k) {
        map<TreeNode*, vector<TreeNode*>> adj;
        queue<TreeNode*> q;
        q.push(root);
        while(!q.empty()) {
            auto node = q.front();
            q.pop();
            if(node->left) {
                adj[node].push_back(node->left);
                adj[node->left].push_back(node);
                q.push(node->left);
            }
            if(node->right) {
                adj[node].push_back(node->right);
                adj[node->right].push_back(node);
                q.push(node->right);
            }
        }
        map<TreeNode*, bool> vis;
        q.push(target);
        while(k--) {
            int n = q.size();
            while(n--) {
                auto u = q.front();
                q.pop();
                if(vis[u]) continue;
                vis[u] = true;
                for(auto v: adj[u]) q.push(v);
            }
        }
        vector<int> ret;
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            if(vis[u]) continue;
            vis[u] = true;
            ret.push_back(u->val);
        }
        return ret;
    }
};