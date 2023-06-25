class Solution {
public:
    int gcd(int a, int b) {
       if (b == 0)
       return a;
       return gcd(b, a % b);
    }
    int countBeautifulPairs(vector<int>& nums) {
        int ret = 0;
        int n = nums.size();
        for(int i = 0;i<n;i++) {
            int a = nums[i];
            while(a/10 != 0) {
                a /= 10;
            }
            for(int j = i+1;j<n;j++) {
                int b = nums[j]%10;
                ret += gcd(a,b) == 1;
            }
        }
        return ret;
    }
};