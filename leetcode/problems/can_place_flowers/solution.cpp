class Solution {
public:
    bool canPlaceFlowers(vector<int>& flowerbed, int n) {
        int m = flowerbed.size();
        for(int i = 0;i<m;i++) {
            if(flowerbed[i]) continue;
            bool l = i == 0 || !flowerbed[i-1];
            bool r = i == m-1 || !flowerbed[i+1];
            if(l && r) n--, flowerbed[i] = 1;
        }
        return n<=0;
    }
};