/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
#define INF 1001
class Codec {
public:

    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        if(!root) {
            return "";
        }
        stringstream ss;
        ss<<root->val;
        ss<<"("<<serialize(root->left)<<")";
        ss<<"("<<serialize(root->right)<<")";
        return ss.str();
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        if(data.empty()) {
            return nullptr;
        }
        //cout<<"data = "<<data<<endl;
        stringstream ss(data);
        int val;
        ss>>val;
        int count = 0;
        char c;
        int i = ss.tellg();
        int j = i;
        while(true) {
            if(data[j]=='(') {
                count++;
            } else if(data[j]==')') {
                count--;
            }
            if(count == 0) {
                break;
            }
            j++;
        }
        string left = data.substr(i+1, j - i - 1);
        //cout<<"left = "<<left<<endl;
        i = j+1;
        j = data.size() - 1;
        string right = data.substr(i+1, j - i - 1);
        //cout<<"right = "<<right<<endl;
        auto root = new TreeNode(val);
        root->left = deserialize(left);
        root->right = deserialize(right);
        return root;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec ser, deser;
// TreeNode* ans = deser.deserialize(ser.serialize(root));