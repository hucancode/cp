class DataStream {
    int target;
    int count;
    int key;
public:
    DataStream(int value, int k) {
        target = k;
        key = value;
        count = 0;
    }
    
    bool consec(int num) {
        if(num == key) {
            count++;
        } else {
            count = 0;
        }
        if(count >= target) {
            return true;
        }
        return false;
    }
};

/**
 * Your DataStream object will be instantiated and called as such:
 * DataStream* obj = new DataStream(value, k);
 * bool param_1 = obj->consec(num);
 */