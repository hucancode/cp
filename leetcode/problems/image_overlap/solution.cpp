class Solution {
public:
    int overlap(vector<vector<int>>& img1, vector<vector<int>>& img2, int ox, int oy) {
        int ret = 0;
        int n = img1.size();
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<n;j++) {
                int i1 = i+ox;
                int j1 = j+oy;
                if(i1<0 || i1>=n || j1<0 || j1>=n) {
                    continue;
                }
                if(img1[i1][j1] && img2[i][j]) {
                    ret++;
                }
            }
        }
        return ret;
    }
    int largestOverlap(vector<vector<int>>& img1, vector<vector<int>>& img2) {
        int n = img1.size();
        int ret = 0;
        for(int ox=-n+1;ox<n;ox++) {
            for(int oy=-n+1;oy<n;oy++) {
                ret = max(ret, overlap(img1, img2, ox, oy));
            }
        }
        return ret;
    }
};