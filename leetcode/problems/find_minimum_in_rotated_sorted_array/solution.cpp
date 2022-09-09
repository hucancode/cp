class Solution {
public:
    int findMin(vector<int>& a) {
        int l = 0;
        int r = a.size() - 1;
        while(a[l] > a[r]) {
            int m = (r + l)/2;
            if(a[m] >= a[l]) {
                l = m+1;
            } else {
                r = m;
            }
        }
        return a[l];
    }
};