class MinStack {
public:
    stack<int> data;
    stack<int> m;
    MinStack() {
        
    }
    
    void push(int val) {
        data.push(val);
        if(!m.empty()) {
            m.push(min(m.top(), val));
        } else {
            m.push(val);
        }
    }
    
    void pop() {
        data.pop();
        m.pop();
    }
    
    int top() {
        return data.top();
    }
    
    int getMin() {
        return m.top();
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