class Solution {
public:
    int totalFruit(vector<int>& fruits) {
        int n = fruits.size();
        int ret = 0;
        int i = 0;
        int j = 0;
        set<int> f;
        while(j < n) {
            f.insert(fruits[j]);
            if(f.size() > 2) {
                j--;
                i = j;
                while(i > 0 && fruits[i-1] == fruits[j]) {
                    i--;
                }
                f.clear();
                f.insert(fruits[j]);
            }
            ret = max(ret, j-i+1);
            j++;
        }
        return ret;
    }
};