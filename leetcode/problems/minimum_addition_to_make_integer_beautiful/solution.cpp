class Solution {
public:
    long long makeIntegerBeautiful(long long n, int target) {
        int sum = 0;
        auto m = n;
        while(m > 0) {
            sum += m%10;
            m/=10;
        }
        m = n;
        int remainder = 0;
        long long ret = 0;
        long long base = 1;
        //cout<<"__________"<<endl;
        while(sum > target || remainder != 0) {
            int d = m%10;
            //cout<<"check "<<d<<" sum = "<<sum<<endl;
            if(d + remainder == 10) {
                sum -= d;
            } else {
                sum += remainder;
                d += remainder;
                if(sum <= target) {
                    break;
                }
                sum -= d;
                ret += base * (10 - d);
                remainder = 1;
            }
            m /= 10;
            base *= 10;
        }
        return ret;
    }
};