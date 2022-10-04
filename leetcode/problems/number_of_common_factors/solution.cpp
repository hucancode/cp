class Solution {
public:
    int gcd(int a, int b) {
        while(b) {
          int tmp = b;
          b = a%b;
          a = tmp;
        }
        return a;
    }
    int commonFactors(int a, int b) {
        int k = gcd(a,b);
        if(k == 1) {
            return 1;
        }
        int ret = 2;
        int n = sqrt(k);
        if(n*n == k) {
            ret++;
            n--;
        }
        for(int i = 2;i<=n;i++) {
            if(k%i==0) {
                ret+=2;
            }
        }
        return ret;
    }
};