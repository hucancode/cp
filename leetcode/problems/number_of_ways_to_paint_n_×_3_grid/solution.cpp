class Solution {
public:
    int numOfWays(int n) {
        const int INF = 1e9+7;
        int xyz = 6;
        int xyx = 6;
        for(int i = 1;i<n;i++) {
            int nextXYX = (xyx*3LL+xyz*2LL)%INF;
            int nextXYZ = (xyx*2LL+xyz*2LL)%INF;
            xyz = nextXYZ;
            xyx = nextXYX;
        }
        return (xyx+xyz)%INF;
    }
};