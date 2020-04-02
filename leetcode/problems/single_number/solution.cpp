class Solution {
public:
    int singleNumber(vector<int>& nums) {
        set<int> a;
        for(int num: nums)
        {
            auto i = a.find(num);
            if(i != a.end())
            {
                a.erase(i);
            }
            else
            {
                a.insert(num);
            }
        }
        return *a.begin();
    }
};