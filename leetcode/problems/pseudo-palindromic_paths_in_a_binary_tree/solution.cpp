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
    void print(vector<int>& counts) {
        for(int i = 0;i<counts.size();i++) {
            if(counts[i] == 0) {
                continue;
            }
            cout<<"key "<<i<<" count "<<counts[i]<<endl; 
        }
    }
    bool palindrome(vector<int>& counts) {
        int odd = 0;
        for(const auto& x: counts) {
            if(x%2 == 1) {
                odd++;
                if(odd > 1) {
                    return false;
                }
            }
        }
        return true;
    }
    int pseudoPalindromicPaths (TreeNode* root) {
        if(!root) {
            return 0;
        }
        set<TreeNode*> vis;
        stack<TreeNode*> st;
        st.push(root);
        vector<int> counts(10, 0);
        int ret = 0;
        while(!st.empty()) {
            auto node = st.top();
            st.pop();
            if(vis.find(node) != vis.end()) {
                counts[node->val]--;
                continue;
            }
            //cout<<"check "<<node->val<<endl;
            counts[node->val]++;
            //print(counts);
            vis.insert(node);
            st.push(node);
            if(palindrome(counts) && !node->left && !node->right) {
                //cout<<"palindrome !"<<endl;
                ret++;
            }
            if(node->right) {
                st.push(node->right);
            }
            if(node->left) {
                st.push(node->left);
            }
        }
        return ret;
    }
};