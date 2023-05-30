#define INF 100003
class MyHashSet {
    vector<bool> data;
    int hash(int key) {
        return key%data.size();
    }
public:
    MyHashSet() {
        data.resize(INF, false);
    }
    void add(int key) {
        data[hash(key)] = true;
    }
    void remove(int key) {
        data[hash(key)] = false;
    }
    bool contains(int key) {
        return data[hash(key)];
    }
};

/**
 * Your MyHashSet object will be instantiated and called as such:
 * MyHashSet* obj = new MyHashSet();
 * obj->add(key);
 * obj->remove(key);
 * bool param_3 = obj->contains(key);
 */