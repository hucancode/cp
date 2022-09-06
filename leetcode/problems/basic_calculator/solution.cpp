class Solution {
public:
    stack<int> v;
    stack<char> op; 
    void applyLastOp() {
        if(op.empty()) {
            return;
        }
        if(op.top() == '(') {
            return;
        }
        char o = op.top();
        op.pop();
        if(o == '+') {
            int y = v.top();
            v.pop();
            int x = v.top();
            v.pop();
            v.push(x+y);
        } else if(o == '-') {
            int y = v.top();
            v.pop();
            int x = v.top();
            v.pop();
            v.push(x-y);
        }
    }
    int calculate(string s) {
        bool unary = true;
        for(int i = 0;i<s.size();i++) {
            if(s[i] == '(') {
                op.push(s[i]);
                unary = true;
                continue;
            }
            if(s[i] == '+') {
                applyLastOp();
                op.push(s[i]);
                continue;
            }
            if(s[i] == '-') {
                if(unary) {
                    v.push(0);
                }
                applyLastOp();
                op.push(s[i]);
                continue;
            }
            if(s[i] == ')') {
                while(op.top() != '(') {
                    applyLastOp();
                }
                op.pop();
                unary = false;
                continue;
            }
            if(s[i] <= '9' && s[i] >= '0') {
                int x = 0;
                while(i < s.size() && (s[i] <= '9' && s[i] >= '0')) {
                    x = (x*10) + (s[i]-'0');
                    i++;
                }
                i--;
                v.push(x);
                unary = false;
            }
        }
        while(!op.empty()) {
            applyLastOp();
        }
        return v.top();
    }
};