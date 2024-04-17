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
    string smallestFromLeaf(TreeNode* root) {
        auto smaller = [](string& a, string& b) {
            for(auto i = a.rbegin(), j = b.rbegin();i != a.rend() && j != b.rend();i++,j++) {
                if(*i == *j) {
                    continue;
                }
                return *i < *j;
            }
            return a.size() < b.size();
        };
        string ret = "";
        string current = "";
        vector<TreeNode*> st = {root};
        while(!st.empty()) {
            auto node = st.back();
            st.pop_back();
            if(!node) {
                current.pop_back();
                continue;
            }
            st.push_back(nullptr);
            current.push_back(node->val + 'a');
            if(!node->left && !node->right) {
                if(ret.empty() || smaller(current, ret)) {
                    ret = current;
                }
                continue;
            }
            if(node->left) {
                st.push_back(node->left);
            }
            if(node->right) {
                st.push_back(node->right);
            }
        }
        reverse(ret.begin(), ret.end());
        return ret;
    }
};
