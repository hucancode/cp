class MyQueue {
public:
    stack<int> input;
    stack<int> output;
    MyQueue() {
    }
    
    void push(int x) {
        input.push(x);
    }
    
    void transfer() {
        if(output.empty()) {
            while(!input.empty()) {
                output.push(input.top());
                input.pop();
            }
        }
    }
    
    int pop() {
        transfer();
        int ret = output.top();
        output.pop();
        return ret;
    }
    
    int peek() {
        transfer();
        return output.top();
    }
    
    bool empty() {
        return input.empty() && output.empty();
    }
};

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue* obj = new MyQueue();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->peek();
 * bool param_4 = obj->empty();
 */