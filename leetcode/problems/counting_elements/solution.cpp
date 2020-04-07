class Solution {
public:
    int countElements(vector<int>& arr)
    {
        if(arr.size() < 2)
        {
            return 0;
        }
        sort(arr.begin(),arr.end());
        int last = arr[0];
        int c = 1;
        int ret = 0;
        for(int i=1;i<arr.size();i++)
        {
            if(arr[i] == last)
            {
                c++;
                continue;
            }
            if(arr[i] == last + 1)
            {
                ret += c;
            }
            last = arr[i];
            c = 1;
        }
        return ret;
    }
};