typedef long long ll;
class Solution {
public:
    int evalRPN(vector<string>& tokens) {
        stack<ll> s;
        for(int i=0;i<tokens.size();i++) {
            auto token = tokens[i];
            if(token == "+") {
                auto a = s.top();
                s.pop();
                auto b = s.top();
                s.pop();
                //cout<<"push "<<a<<"+"<<b<<endl;
                s.push(a+b);
            } else if(token == "-") {
                auto a = s.top();
                s.pop();
                auto b = s.top();
                s.pop();
                s.push(b-a);
                //cout<<"push "<<a<<"-"<<b<<endl;
            } else if(token == "*") {
                auto a = s.top();
                s.pop();
                auto b = s.top();
                s.pop();
                s.push(a*b);
                //cout<<"push "<<a<<"*"<<b<<endl;
            } else if(token == "/") {
                auto a = s.top();
                s.pop();
                auto b = s.top();
                s.pop();
                s.push(b/a);
                //cout<<"push "<<a<<"/"<<b<<endl;
            } else {
                stringstream ss(token);
                int k;
                ss>>k;
                //cout<<"push "<<k<<endl;
                s.push(k);
            }
        }
        return s.top();
    }
};