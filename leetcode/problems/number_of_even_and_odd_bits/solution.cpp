class Solution {
public:
    vector<int> evenOddBit(int n) {
        int a = 0;
        int b = 0;
        bool even = true;
        while(n != 0) {
            if(even) {
                a += n&1;
            } else {
                b += n&1;
            }
            n = n>>1;
            even = !even;
        }
        return {a, b};
    }
};