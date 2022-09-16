class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int n = matrix.size();
        int m = matrix[0].size();
        auto cmpHead = [](int a, const vector<int>& b){
            return a < *b.begin();
        };
        auto right = upper_bound(matrix.begin(), matrix.end(), target, cmpHead);
        if(right == matrix.begin()) {
            return false;
        }
        auto cmpTail = [](int a, const vector<int>& b){
            return a < *b.rbegin();
        };
        auto left = upper_bound(matrix.begin(), matrix.end(), target, cmpTail);
        if(left != matrix.begin()) {
            left--;
        }
        for(auto it = left;it != right;it++) {
            if(binary_search(it->begin(), it->end(), target)) {
                return true;
            }
        }
        return false;
    }
};