class BrowserHistory {
    stack<string> next;
    stack<string> previous;
public:
    BrowserHistory(string homepage) {
        previous.push(homepage);
    }
    
    void visit(string url) {
        while(!next.empty()) next.pop();
        previous.push(url);
    }
    
    string back(int steps) {
        while(steps-- && previous.size() > 1) {
            next.push(previous.top());
            previous.pop();
        }
        return previous.top();
    }
    
    string forward(int steps) {
        while(steps-- && !next.empty()) {
            previous.push(next.top());
            next.pop();
        }
        return previous.top();
    }
};

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * BrowserHistory* obj = new BrowserHistory(homepage);
 * obj->visit(url);
 * string param_2 = obj->back(steps);
 * string param_3 = obj->forward(steps);
 */