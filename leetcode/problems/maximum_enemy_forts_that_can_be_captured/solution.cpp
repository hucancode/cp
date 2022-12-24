class Solution {
public:
    int captureForts(vector<int>& forts) {
        int n = forts.size();
        int ret = 0;
        int j = -1;
        for(int i = 0;i<n;i++) {
            if(forts[i] == 0) {
                continue;
            }
            if(j != -1 && forts[j] != forts[i]) {
                ret = max(ret, abs(i-j-1));
            }
            j = i;
        }
        return ret;
    }
};