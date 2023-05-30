#define INF 100003
class MyHashMap {
    vector<int> data;
    int hash(int key) {
        return key % data.size();
    }
public:
    MyHashMap() {
        data.resize(INF, -1);
    }
    
    void put(int key, int value) {
        data[hash(key)] = value;
    }
    
    int get(int key) {
        return data[hash(key)];
    }
    
    void remove(int key) {
        data[hash(key)] = -1;
    }
};

/**
 * Your MyHashMap object will be instantiated and called as such:
 * MyHashMap* obj = new MyHashMap();
 * obj->put(key,value);
 * int param_2 = obj->get(key);
 * obj->remove(key);
 */