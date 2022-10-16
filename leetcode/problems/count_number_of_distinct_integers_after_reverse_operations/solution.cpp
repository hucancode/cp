class Solution {
public:
    int invert(int n) {
        int remainder;
        int ret = 0;
        while (n != 0) {
            remainder = n % 10;
            ret = ret * 10 + remainder;
            n /= 10;
        }
        return ret;
    }
    int countDistinctIntegers(vector<int>& nums) {
        set<int> s;
        for(auto x: nums) {
            s.insert(x);
            s.insert(invert(x));
        }
        return s.size();
    }
};