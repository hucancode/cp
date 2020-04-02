class Solution {
public:
    bool isHappy(int n)
    {
        bool happy[1000];
        memset(happy,false,sizeof(happy));
        bool seen[1000];
        memset(seen,false,sizeof(seen));
        happy[1] = true;
        happy[7] = true;
        happy[13] = true;
        happy[31] = true;
        happy[19] = true;
        happy[91] = true;
        happy[28] = true;
        happy[82] = true;
        happy[49] = true;
        happy[94] = true;
        happy[68] = true;
        happy[86] = true;
        while(true)
        {
            int sum = 0;
            while(n!= 0)
            {
                sum += pow(n%10,2);
                n /= 10;
            }
            if(happy[sum])
            {
                return true;
            }
            if(seen[sum])
            {
                break;
            }
            seen[sum] = true;
            n = sum;
        }
        return false;
    }
};