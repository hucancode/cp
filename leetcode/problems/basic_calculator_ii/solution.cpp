class Solution {
public:
    bool pred(char a, char b) {
        if(a == '*' || a == '/') {
            return true;
        }
        if(b == '-' || b == '+') {
            return true;
        }
        return false;
    }
    int calculate(int a, int b, char op) {
        //cout<<"calculate "<<a<<op<<b<<endl;
        if(op == '+') {
            return a+b;
        }
        if(op == '-') {
            return a-b;
        }
        if(op == '*') {
            return a*b;
        }
        if(op == '/') {
            return a/b;
        }
        return a;
    }
    int calculate(string s) {
        stack<char> op;
        stack<int> ret;
        int x = 0;
        for(const auto& c: s) {
            if(c == ' ') {
                continue;
            }
            if(c <= '9' && c>= '0') {
                auto d = c-'0';
                x = x*10 + d;
                continue;
            }
            //cout<<"see "<<c<<endl;
            ret.push(x);
            while(!op.empty() && pred(op.top(), c)) {
                auto b = ret.top();
                ret.pop();
                auto a = ret.top();
                ret.pop();
                auto o = op.top();
                op.pop();
                ret.push(calculate(a, b, o));
            }
            op.push(c);
            //cout<<"push "<<x<<", "<<c<<endl;
            x = 0;
        }
        ret.push(x);
        while(!op.empty()) {
            auto b = ret.top();
            ret.pop();
            auto a = ret.top();
            ret.pop();
            auto o = op.top();
            op.pop();
            ret.push(calculate(a, b, o));
        }
        return ret.top();
    }
};