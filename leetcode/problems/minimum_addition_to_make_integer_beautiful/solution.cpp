class Solution {
public:
    int sumDigit(long long n) {
        int ret = 0;
        long long k = 1e12;
        while(n!= 0) {
            ret += n/k;
            n = n%k;
            k/=10;
        }
        return ret;
    }
    long long makeIntegerBeautiful(long long n, int target) {
        int sum = sumDigit(n);
        if(sum <= target) {
            return 0;
        }
        long long ret = 0;
        long long k = 1;
        int remaining = 0;
        //cout<<"initial sum = "<<sum<<endl;
        while(sum+remaining > target) {
            int d = n%10;
            sum -= d;
            int a = max(0,10-d-remaining);
            //cout<<"d "<<d<<" a "<<a<<" r "<<remaining<<endl;
            remaining = 1;
            ret += a*k;
            n /= 10;
            k *= 10;
        }
        return ret;
    }
};