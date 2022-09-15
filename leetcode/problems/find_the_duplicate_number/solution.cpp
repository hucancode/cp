class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        int n = nums.size() - 1;
        int l = 1;
        int r = n;
        while(l <= r) {
            int m = (r+l)/2;
            int lcount = 0;
            int rcount = 0;
            for(const auto& x: nums) {
                if(x < m) {
                    lcount++;
                } else if(x > m) {
                    rcount++;
                }
            }
            //cout<<"key = "<<m<<", left = "<<lcount<<", right = "<<rcount<<endl;
            //cout<<"supposed left = "<<m-1<<", right = "<<n - m<<endl;
            if(lcount > m-1) {
                //cout<<"result on the left"<<endl;
                r = m-1;
                continue;
            }
            if(rcount > n-m) {
                //cout<<"result on the right"<<endl;
                l = m+1;
                continue;
            }
            return m;
        }
        return 0;
    }
};