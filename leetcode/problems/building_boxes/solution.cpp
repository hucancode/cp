class Solution {
public:
    int canPut(int level) {
        int ret = 0;
        for(int i = 1;i<=level;i++) {
            ret += i*(level-i+1);
        }
        return ret;
    }
    int floorBox(int level) {
        return level*(level+1)/2;
    }
    int minimumBoxes(int n) {
        if(n < 4) {
            return n;
        }
        int MAX_LEVEL = 1818;// canPut(1818) = 1,003,105,740
        int l = 1, r = MAX_LEVEL;
        while(l < r) {
            //cout<<"search "<<l<<"-"<<r<<endl;
            int m = (l+r)/2;
            if(canPut(m) >= n) {
                r = m;
            } else {
                l = m+1;
            }
        }
        //cout<<"need "<<l<<" level to put "<<n<<" box"<<endl;
        //cout<<"canput("<<l<<") ="<<canPut(l)<<endl;
        //cout<<"canput("<<l-1<<") ="<<canPut(l-1)<<endl;
        int ret = floorBox(l);
        //cout<<l<<"level would have "<<ret<<" box on the floor"<<endl;
        int m = canPut(l);
        int k = l;
        while(m - k >= n) {
            ret--;
            m -= k--;
        }
        return ret;
    }
};