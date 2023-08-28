class MyStack {
    queue<int> data;
    int _top;
public:
    MyStack() {
        
    }
    
    void push(int x) {
        data.push(x);
        _top = x;
    }
    
    int pop() {
        int ret = _top;
        int n = data.size() - 1;
        queue<int> next;
        while(n--) {
            int x = data.front();
            next.push(x);
            data.pop();
            _top = x;
        }
        data = next;
        return ret;
    }
    
    int top() {
        return _top;
    }
    
    bool empty() {
        return data.empty();
    }
};

/**
 * Your MyStack object will be instantiated and called as such:
 * MyStack* obj = new MyStack();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->top();
 * bool param_4 = obj->empty();
 */