class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& a) {
        sort(a.begin(), a.end(), 
            [](const vector<int>& a, const vector<int>& b) -> bool
        { 
            return a[0] < b[0]; 
        });
        int l = 0;
        int r = 1;
        while(l < a.size() - 1) {
            r = l;
            while(r < a.size() - 1) {
                if(a[r+1][0] > a[l][1]) {
                    break;
                }
                r++;
                a[l][1] = max(a[l][1], a[r][1]);
            }
            if(r > l) {
                a.erase(a.begin()+l+1, a.begin()+r+1);
            }
            l++;
        }
        return a;
    }
};