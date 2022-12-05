class MyCalendar {
    vector<pair<int,int>> data;
public:
    MyCalendar() {
        
    }
    bool overlap(int start, int end) {
        pair<int,int> target = {end, end};
        auto right = upper_bound(data.begin(), data.end(), target);
        for(auto it = data.begin(); it != right; it++) {
            if(it->second > start) {
                return true;
            }
        }
        return false;
    }
    bool book(int start, int end) {
        if(overlap(start, end)) {
            return false;
        }
        pair<int,int> target = {start, end};
        auto it = lower_bound(data.begin(), data.end(), target);
        data.insert(it, target);
        return true;
    }
};

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */