class Solution {
public:
    int averageValue(vector<int>& nums) {
        int sum = 0;
        int count = 0;
        for(auto x: nums) {
            if(x%3==0 && x%2==0) {
                sum+=x;
                count++;
            }
        }
        if(count ==0)
            return 0;
        return sum/count;
    }
};