class Solution {
public:
    bool isOne(int num, int bit) {
        int k = 1<<bit;
        return (num & k) != 0;
    }
    bool isZero(int num, int bit) {
        int k = 1<<bit;
        return (num & k) == 0;
    }
    void turnOn(int& num, int bit) {
        int k = 1<<bit;
        num = num | k;
    }
    void turnOff(int& num, int bit) {
        int k = 0xffffffff ^ (1<<bit);
        num = num & k;
    }
    int countBit1(int num) {
        int ret = 0;
        for(int i = 0;i<31;i++) {
            if(1<<i & num) {
                ret++;
            }
        }
        return ret;
    }
    int minimizeXor(int num1, int num2) {
        int n2bit = countBit1(num2);
        int n1bit = countBit1(num1);
        int ret = num1;
        int d = n2bit - n1bit;
        for(int i = 0;i<32;i++) {
            if(d == 0) {
                break;
            }
            if(isZero(num1, i) && d > 0) {
                turnOn(ret, i);
                d--;
            }
            if(isOne(num1, i) && d < 0) {
                turnOff(ret, i);
                d++;
            }
            cout<<"num1 = "<<num1<<endl;
        }
        return ret;
    }
};