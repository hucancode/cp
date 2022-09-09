class Solution {
public:
    
    int binarySearch(vector<int>& a) {
        int l = 0;
        int r = a.size() - 1;
        while(a[l] >= a[r] && l < r) {
            int m = (r + l)/2;
            if(a[m] == a[l]) {
                l++;
                continue;
            }
            if(a[m] == a[r]) {
                r--;
                continue;
            }
            if(a[m] > a[l]) {
                l = m+1;
            } else {
                r = m;
            }
        }
        return a[l];
    }
    int linearSearch(vector<int>& a) {
        int i = 0;
        int n = a.size();
        if(a[0] < a[n-1]) {
            return a[0];
        }
        while(i < n - 1) {
            if(a[i+1] < a[i]) {
                return a[i+1];
            }
            i++;
        }
        return a[i];
    }
    int findMin(vector<int>& a) {
        return binarySearch(a);
    }
};