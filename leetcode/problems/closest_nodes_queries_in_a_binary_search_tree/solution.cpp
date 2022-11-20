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
    vector<int> arr;
    void dfs(TreeNode* root) {
        if(!root) {
            return;
        }
        dfs(root->left);
        arr.push_back(root->val);
        dfs(root->right);
    }
    vector<vector<int>> closestNodes(TreeNode* root, vector<int>& queries) {
        dfs(root);
        vector<vector<int>> ret;
        for(auto q: queries) {
            auto a = lower_bound(arr.begin(), arr.end(), q);
            if(a == arr.end() || (a != arr.begin() && *a > q)) {
                a--;
            }
            if(*a == q) {
                ret.push_back({q,q});
                continue;
            }
            auto b = upper_bound(arr.begin(), arr.end(), q);
            vector<int> ans = {
                *a < q?*a:-1,
                b == arr.end()?-1:*b,
            };
            ret.push_back(move(ans));
        }
        return ret;
    }
};