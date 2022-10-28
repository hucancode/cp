class Solution {
public:
    vector<string> topKFrequent(vector<string>& words, int k) {
        map<string, int> mp;
        for(auto& word: words) {
            mp[word]++;
        }
        typedef pair<int, string> state;
        priority_queue<state> q;
        for(auto& item: mp) {
            q.emplace(item.second, item.first);
        }
        vector<string> ret;
        vector<string> buffer;
        int n = q.top().first;
        while(ret.size() < k) {
            while(!q.empty() && q.top().first == n) {
                buffer.emplace_back(q.top().second);
                q.pop();
            }
            if(!q.empty()) {
                n = q.top().first;
            }
            sort(buffer.begin(), buffer.end());
            ret.insert(ret.end(), buffer.begin(), buffer.end());
            buffer.clear();
            if(ret.size() >= k) {
                ret.resize(k);
                break;
            }
        }
        return ret;
    }
};