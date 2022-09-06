class FreqStack {
public:
    map<int, int> freq;
    map<int, stack<int>> group;
    int fmax;
    FreqStack() {
        fmax = 0;
    }
    
    void push(int val) {
        if(freq.find(val) == freq.end()) {
            freq[val] = 0;
        }
        freq[val]++;
        fmax = max(fmax, freq[val]);
        group[freq[val]].push(val);
    }
    
    int pop() {
        auto target = group[fmax].top();
        freq[target]--;
        group[fmax].pop();
        if(group[fmax].empty()) {
            group.erase(group.find(fmax));
            fmax--;
        }
        return target;
    }
};

/**
 * Your FreqStack object will be instantiated and called as such:
 * FreqStack* obj = new FreqStack();
 * obj->push(val);
 * int param_2 = obj->pop();
 */