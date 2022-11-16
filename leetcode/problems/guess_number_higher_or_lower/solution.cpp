/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * int guess(int num);
 */

class Solution {
public:
    int guessNumber(int n) {
        unsigned int l = 0, r = n;
        while(l<r) {
            int m = (l+r)/2;
            int ret = guess(m);
            if(ret == 0) {
                l = m;
                break;
            }
            if(ret == 1) {
                l = m+1;
                continue;
            }
            if(ret == -1) {
                r = m;
                continue;
            }
        }
        return l;
    }
};