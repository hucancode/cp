class Solution {
public:
    bool check(vector<string>& a, int x, int y) {
        int n = a.size();
        for(int i = 0;i<n;i++) {
            if(a[x][i] == 'Q' || a[i][y] == 'Q') {
                return false;
            }
            if(x + i < n) {
                if(y + i < n) {
                    if(a[x+i][y+i] =='Q') {
                        return false;
                    }
                }
                if(y - i >= 0) {
                    if(a[x+i][y-i] =='Q') {
                        return false;
                    }
                }
            }
            if(x - i >= 0) {
                if(y + i < n) {
                    if(a[x-i][y+i] =='Q') {
                        return false;
                    }
                }
                if(y - i >= 0) {
                    if(a[x-i][y-i] =='Q') {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    int totalNQueens(int n) {
        int ret = 0;
        vector<string> a(n, string(n, '.'));
        stack<pair<pair<int, int>, char>> st;
        for(int i = 0;i<n;i++) {
            st.push(make_pair(make_pair(0, i), 'Q'));
        }
        while(!st.empty()) {
            auto candidate = st.top();
            st.pop();
            auto x = candidate.first.first;
            auto y = candidate.first.second;
            auto value = candidate.second;
            a[x][y] = value;
            if(value == '.') {
                //cout<<"revert at "<<x<<"-"<<y<<endl;
                continue;
            }
            //cout<<"try placing at "<<x<<"-"<<y<<endl;
            st.push(make_pair(make_pair(x, y), '.'));
            x++;
            if(x == n) {
                ret++;
                //cout<<"record solution"<<endl;
                continue;
            }
            for(y = 0;y<n;y++) {
                if(!check(a, x, y)) {
                    continue;
                }
                //cout<<"pushing next candidate "<<x<<"-"<<y<<endl;
                st.push(make_pair(make_pair(x, y), 'Q'));
            }
        }
        return ret;
    }
};