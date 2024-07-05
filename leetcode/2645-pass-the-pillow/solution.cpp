class Solution {
public:
    int passThePillow(int n, int time) {
        int d = time/(n-1)%2;
        int i = time%(n-1);
        return d == 0?(i+1):(n-i);
    }
};
