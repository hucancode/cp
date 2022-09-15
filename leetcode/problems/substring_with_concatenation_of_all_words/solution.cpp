class Solution {
public:
    vector<int> findSubstring(string s, vector<string>& words) {
        int n = words.size();
        int m = words[0].size();
        int len = n*m;
        sort(words.begin(), words.end());
        vector<bool> vis(n, false);
        int i = 0;
        vector<int> ret;
        while(s.size() - i >= len) {
            //cout<<"check "<<s.substr(i,len)<<endl;
            fill(vis.begin(), vis.end(), false);
            bool good = true;
            for(int j = 0;j<len;j+=m) {
                string key = s.substr(i+j,m);
                auto k = distance(words.begin(), lower_bound(words.begin(), words.end(), key));
                while(vis[k] && k < words.size()) {
                    //cout<<"vis["<<k<<"] = true, go to next word"<<endl;
                    k++;
                }
                if(k >= words.size()) {
                    //cout<<key<<" not found, break"<<endl;
                    good = false;
                    break;
                }
                if(words[k] != key) {
                    //cout<<key<<" not found, break"<<endl;
                    good = false;
                    break;
                }
                vis[k] = true;
            }
            if(good) {
                ret.push_back(i);
            }
            i++;
        }
        return ret;
    }
};