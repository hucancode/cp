class Solution
{
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs)
    {
        unordered_map<string, vector<string>> hash;
        for(auto& str:strs)
        {
            string key = str;
            sort(key.begin(), key.end());
            hash[key].push_back(str);
        }
        vector<vector<string>> ret;
        ret.reserve(hash.size());
        for(auto& item: hash)
        {
            ret.push_back(item.second);
        }
        return ret;
    }
    
};