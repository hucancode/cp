class SORTracker {
public:
    set<pair<int, string>> s;
    set<pair<int, string>>::iterator cursor;
    SORTracker() {
        cursor = s.end();
    }
    void add(string name, int score) {
        auto it = s.insert({-score, name}).first;
        auto newElementCameBefore = *it < *cursor;
        if (cursor == s.end() || newElementCameBefore) {
            cursor--;
        }
    }
    string get() {
        return (cursor++)->second;
    }
};

/**
 * Your SORTracker object will be instantiated and called as such:
 * SORTracker* obj = new SORTracker();
 * obj->add(name,score);
 * string param_2 = obj->get();
 */