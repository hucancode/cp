class MinStack {
public:
    stack<int> items;
    stack<int> mins;
    MinStack() {
        
    }
    
    void push(int val) {
        items.push(val);
        if(mins.empty()) {
            mins.push(val);
        } else {
            mins.push(min(mins.top(), val));
        }
    }
    
    void pop() {
        items.pop();
        mins.pop();
    }
    
    int top() {
        return items.top();
    }
    
    int getMin() {
        return mins.top();
    }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(val);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */