class Solution {
public:
    vector<vector<string>> findDuplicate(vector<string>& paths) {
        map<string, vector<string>> mp;
        for(const auto& path: paths) {
            stringstream ss(path);
            string dir;
            ss>>dir;
            string file;
            while(ss>>file) {
                file.resize(file.size() - 1);
                int k = file.find('(');
                string name = file.substr(0,k);
                string content = file.substr(k+1);
                //cout<<dir+"/"+name<<" - "<<content<<endl;
                mp[content].push_back(dir+"/"+name);
            }
        }
        vector<vector<string>> ret;
        for(const auto& item: mp) {
            if(item.second.size() <= 1) {
                continue;
            }
            ret.push_back(item.second);
        }
        return ret;
    }
};