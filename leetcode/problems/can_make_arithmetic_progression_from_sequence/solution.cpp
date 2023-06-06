class Solution {
public:
    bool canMakeArithmeticProgression(vector<int>& arr) {
        int n = arr.size();
        int min_a = *min_element(arr.begin(), arr.end());
        int max_a = *max_element(arr.begin(), arr.end());
        if((max_a - min_a)%(n-1) != 0) {
            return false;
        }
        int d = (max_a - min_a)/(n-1);
        if(d == 0) {
            return true;
        }
        vector<bool> vis(n, false);
        for(int x: arr) {
            int i = (x-min_a)/d;
            vis[i] = true;
        }
        return accumulate(vis.begin(), vis.end(), true, multiplies<bool>());
    }
};