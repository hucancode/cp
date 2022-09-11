class Solution {
public:
    string add(string num1, string num2) {
        int carry = 0;
        int i = num1.size() - 1;
        int j = num2.size() - 1;
        string ret;
        while(i >= 0 || j >= 0) {
            int a = (i >= 0)?(num1[i] - '0'):0;
            int b = (j >= 0)?(num2[j] - '0'):0;
            j--;
            i--;
            int c = a+b + carry;
            ret = (char)(c%10 + '0') + ret;
            carry = c/10;
        }
        if(carry != 0) {
            ret = (char)(carry + '0') + ret;
        }
        //cout<<num1<<"+"<<num2<<"="<<ret<<endl;
        return ret;
    }
    string multiply1(string num1, char digit) {
        if(digit == '0' || num1 == "0") {
            return "0";
        }
        int carry = 0;
        int b = digit - '0';
        string ret;
        for(int i = num1.size() - 1 ;i>=0;i--) {
            int a = num1[i] - '0';
            int c = a*b + carry;
            ret = (char)(c%10 + '0') + ret;
            carry = c/10;
        }
        if(carry != 0) {
            ret = (char)(carry + '0') + ret;
        }
        //cout<<num1<<"*"<<digit<<"="<<ret<<endl;
        return ret;
    }
    string multiply(string num1, string num2) {
        string ret = "0";
        string factor = "";
        for(int i = num2.size() - 1;i>=0;i--) {
            string c = multiply1(num1, num2[i]);
            if(c != "0") {
                c += factor;
            }
            ret = add(ret, c);
            factor += "0";
        }
        return ret;
    }
};