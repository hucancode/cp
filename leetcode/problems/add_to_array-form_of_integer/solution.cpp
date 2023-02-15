class Solution {
public:
    vector<int> addToArrayForm(vector<int>& num, int k) {
        int i = num.size() - 1;
        int remainder = 0;
        while(k > 0 || remainder > 0) {
            int a = i<0?0:num[i];
            int b = k%10;
            int x = a+b+remainder;
            remainder = x/10;
            int d = x%10;
            k/=10;
            if(i < 0) {
                num.insert(num.begin(), d);
            } else {
                num[i] = d;
            }
            i--;
        }
        return num;
    }
};