class Solution {
public:
    int closetTarget(vector<string>& words, string target, int startIndex) {
        int ret = -1;
        int n = words.size();
        int i;
        for(i = 0;i<n;i++) {
            if(words[i]==target) {
                int d = abs(i-startIndex);
                if(ret == -1) {
                    ret = min(d,n-d);
                } else {
                    ret = min(ret,min(d,n-d));
                }
            }
        }        
        return ret;
    }
};