class TimeMap {
public:
    map<string, vector<pair<int, string>>> data;
    TimeMap() {
        
    }
    
    void set(string key, string value, int timestamp) {
        auto& a = data[key];
        auto cmp = [](const pair<int, string>& x, int timestamp)
        {
            return x.first < timestamp;
        };
        a.insert(
            lower_bound(a.begin(), a.end(), timestamp, cmp), 
            make_pair(timestamp, value));
    }
    
    string get(string key, int timestamp) {
        auto& a = data[key];
        if(a.empty()) {
            return "";
        }
        int n = a.size();
        auto cmp = [](int timestamp, const pair<int, string>& x)
        {
            return timestamp < x.first;
        };
        auto right = upper_bound(a.begin(), a.end(), timestamp, cmp);
        if(right == a.begin()) {
            return "";
        }
        auto left = right - 1;
        return left->second;
    }
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */