class Solution {
public:
    int costToClear(int bit) {
        return (1<<(bit+1)) - 1;
    }
    int minimumOneBitOperations(int n) {
        vector<int> f(31);// f[i] = cost to make xxxx to be 0000
        vector<int> g(31);// g[i] = cost to make xxxx to be 1000
        f[0] = (n&1)?1:0;
        g[0] = (n&1)?0:1;
        for(int i = 1;i<31;i++) {
            bool isSet = (n&(1<<i)) != 0;
            if(isSet) {
                g[i] = f[i-1];
                f[i] = g[i-1];// cost to make 1xxxx -> 11000
				f[i] += 1;// cost to make 11000 -> 1000
                f[i] += costToClear(i-1);// cost to make 1000 to become 0000
            } else {
                f[i] = f[i-1];
                g[i] = g[i-1];// cost to make 0xxxx -> 01000
				g[i] += 1;// cost to make 01000 -> 11000
                g[i] += costToClear(i-1);// cost to make 1000 to become 0000
            }
        }
        return f[30];
    }
};