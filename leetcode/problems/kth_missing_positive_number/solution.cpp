class Solution {
public:
    int missing(vector<int>& arr, int i) {
        return arr[i] - 1 - i;
    }
    int findKthPositive(vector<int>& arr, int k) {
        int l = 0, r = arr.size() - 1;
        while(l < r) {
            int m = (l+r)/2;
            bool good = missing(arr, m) >= k;
            if(good) {
                r = m;
            } else {
                l = m+1;
            }
        }
        int d = k - missing(arr, l);
        if(d <= 0) {
            d--;
        }
        return d + arr[l];
    }
};