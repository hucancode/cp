class MyCalendarTwo {
    vector<vector<pair<int,int>>> data;
public:
    MyCalendarTwo() {
        data.resize(2);
    }
    bool overlap(int start, int end, int layer) {
        pair<int,int> target = {end, end};
        auto right = upper_bound(data[layer].begin(), data[layer].end(), target);
        for(auto it = data[layer].begin(); it != right; it++) {
            if(it->second > start) {
                return true;
            }
        }
        return false;
    }
    void insert(pair<int,int>& target, int layer) {
        auto it = lower_bound(data[layer].begin(), data[layer].end(), target);
        data[layer].insert(it, target);
    }
    pair<int,int> getOverlap(pair<int,int>& a, pair<int,int>& b) {
        return {max(a.first, b.first), min(a.second,b.second)};
    }
    bool book(int start, int end) {
        if(overlap(start, end, 1)) {
            return false;
        }
        pair<int,int> job = {start,end};
        pair<int,int> target = {end, end};
        auto right = upper_bound(data[0].begin(), data[0].end(), target);
        bool doubleBook = false;
        for(auto it = data[0].begin(); it != right; it++) {
            if(it->second > start) {
                auto next = getOverlap(*it, job);
                if(next.second <= next.first) {
                    continue;
                }
                insert(next,1);
            }
        }
        insert(job,0);
        return true;
    }
};

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */