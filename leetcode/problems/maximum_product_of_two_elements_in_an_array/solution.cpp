class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int a = 0, b = 0;
        for(int x: nums) {
            if(x > a) swap(x,a);
            b = max(x,b);
        }
        return (a-1)*(b-1);
    }
};