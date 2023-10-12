/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 * class MountainArray {
 *   public:
 *     int get(int index);
 *     int length();
 * };
 */

class Solution {
public:
    int findInMountainArray(int target, MountainArray &mountainArr) {
        int n = mountainArr.length();
        int peak = binarySearch(1, n-2, [&](int idx) {
            return mountainArr.get(idx) < mountainArr.get(idx+1);
        });
        int i = binarySearch(0, peak, [&](int idx){
            return mountainArr.get(idx) < target;
        });
        if (mountainArr.get(i) == target) {
            return i;
        }
        int j = binarySearch(peak+1, n-1, [&](int idx){
            return mountainArr.get(idx) > target;
        });
        if (mountainArr.get(j) == target) {
            return j;
        }
        return -1;
    }
    template<typename T>
    int binarySearch(int l, int r, T validator) {
        while(l != r) {
            int m = (l+r)/2;
            bool good = validator(m);
            if (good) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return l;
    }
};