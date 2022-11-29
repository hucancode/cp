class RandomizedSet {
public:
    set<int> data;
    RandomizedSet() {
        
    }
    
    bool insert(int val) {
        if(data.find(val) != data.end()) {
            return false;
        }
        data.insert(val);
        return true;
    }
    
    bool remove(int val) {
        auto it = data.find(val);
        if(it == data.end()) {
            return false;
        }
        data.erase(it);
        return true;
    }
    
    int getRandom() {
        int n = data.size();
        int i = rand()%n;
        auto it = data.begin();
        while(i--) {
            it++;
        }
        return *it;
    }
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */