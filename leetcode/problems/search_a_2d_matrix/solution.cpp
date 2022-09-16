class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        auto cmp = [](int a, const vector<int>& b){
            return a < b[0];
        };
        auto it = upper_bound(matrix.begin(), matrix.end(), target, cmp);
        if(it == matrix.begin()) {
            return false;
        }
        it--;
        return binary_search(it->begin(), it->end(), target);
    }
};