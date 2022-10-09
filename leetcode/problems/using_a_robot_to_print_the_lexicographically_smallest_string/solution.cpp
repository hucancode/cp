class Solution {
public:
    string robotWithString(string s) {
        //string ts;
        string p;
        int n = s.size();
        vector<int> right(n);
        right[n-1] = n-1;
        for(int i = n-2;i>=0;i--) {
            auto prev = right[i+1];
            if(s[i] < s[prev]) {
                right[i] = i;
            } else {
                right[i] = prev;
            }
        }
        // for(int i = 0;i<n;i++) {
        //     cout<<right[i]<<" ";
        // }
        // cout<<endl;
        int i = 0;
        stack<char> t;
        for(int i = 0;i<n;i++) {
            //cout<<"si = "<<s[i]<<", p = "<<p<<", t = "<<ts<<", best si "<<s[right[i]]<<endl;
            while(!t.empty() && t.top() <= s[right[i]]) {
                p.push_back(t.top());
                t.pop();
                //ts.pop_back();
            }
            if(s[i] > s[right[i]]) {
                t.push(s[i]);
                //ts.push_back(s[i]);
            } else {
                p.push_back(s[i]);
            }
        }
        while(!t.empty()) {
            p.push_back(t.top());
            t.pop();
        }
        return p;
    }
};